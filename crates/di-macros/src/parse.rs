//! Parse a derive input into a structured representation.
use syn::{Data, DeriveInput, Fields, spanned::Spanned};

/// Parsed representation of a struct for derive macro generation.
#[derive(Debug)]
pub(crate) struct ParsedStruct {
    /// Struct name.
    pub name: syn::Ident,
    /// Fields resolved from the [`ServiceProvider`].
    pub service_fields: Vec<syn::Ident>,
    /// Fields resolved as trait objects from the [`ServiceProvider`].
    pub trait_fields: Vec<TraitField>,
    /// Fields resolved via [`Default::default()`].
    pub default_fields: Vec<syn::Ident>,
}

/// Field resolved as a trait object from the [`ServiceProvider`].
#[derive(Debug)]
pub(crate) struct TraitField {
    /// Field name.
    pub name: syn::Ident,
    /// Inner trait type, e.g. `dyn HttpFetch`.
    pub trait_type: syn::Type,
}

/// Parse a [`DeriveInput`] into a [`ParsedStruct`].
pub(crate) fn parse_struct(input: &DeriveInput) -> Result<ParsedStruct, syn::Error> {
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            input.generics.span(),
            "FromServices derive does not support generic structs",
        ));
    }
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new(
            input.ident.span(),
            "FromServices derive only supports structs",
        ));
    };
    let (service_fields, trait_fields, default_fields) = match &data.fields {
        Fields::Unit => (Vec::new(), Vec::new(), Vec::new()),
        Fields::Named(fields) => {
            let mut service_fields = Vec::new();
            let mut trait_fields = Vec::new();
            let mut default_fields = Vec::new();
            for field in &fields.named {
                let ident = field.ident.clone().expect("named field should have ident");
                let is_default = has_di_default(&field.attrs)?;
                if is_default {
                    default_fields.push(ident);
                } else if let Some(trait_type) = extract_arc_dyn_type(&field.ty) {
                    trait_fields.push(TraitField {
                        name: ident,
                        trait_type,
                    });
                } else {
                    service_fields.push(ident);
                }
            }
            (service_fields, trait_fields, default_fields)
        }
        other @ Fields::Unnamed(_) => {
            return Err(syn::Error::new(
                other.span(),
                "FromServices derive only supports structs with named fields",
            ));
        }
    };
    Ok(ParsedStruct {
        name: input.ident.clone(),
        service_fields,
        trait_fields,
        default_fields,
    })
}

/// Extract the inner trait type if the field type is `Arc<dyn Trait>`.
fn extract_arc_dyn_type(ty: &syn::Type) -> Option<syn::Type> {
    let syn::Type::Path(type_path) = ty else {
        return None;
    };
    let segment = type_path.path.segments.last()?;
    if segment.ident != "Arc" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    let syn::GenericArgument::Type(inner) = args.args.first()? else {
        return None;
    };
    matches!(inner, syn::Type::TraitObject(_)).then(|| inner.clone())
}

/// Check if a field has the `#[di(default)]` attribute.
fn has_di_default(attrs: &[syn::Attribute]) -> Result<bool, syn::Error> {
    for attr in attrs {
        if !attr.path().is_ident("di") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("default") {
                return Ok(());
            }
            Err(meta.error("unknown di attribute"))
        })?;
        return Ok(true);
    }
    Ok(false)
}
