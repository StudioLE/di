//! Parse a derive input into a structured representation.
use syn::{Data, DeriveInput, Fields, spanned::Spanned};

/// Parsed representation of a struct for derive macro generation.
#[derive(Debug)]
pub(crate) struct ParsedStruct {
    /// Struct name.
    pub name: syn::Ident,
    /// Named field identifiers.
    pub field_names: Vec<syn::Ident>,
}

/// Parse a [`DeriveInput`] into a [`ParsedStruct`].
pub(crate) fn parse_struct(input: &DeriveInput) -> Result<ParsedStruct, syn::Error> {
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            input.generics.span(),
            "FromProvider derive does not support generic structs",
        ));
    }
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields,
            other => {
                return Err(syn::Error::new(
                    other.span(),
                    "FromProvider derive only supports structs with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new(
                input.ident.span(),
                "FromProvider derive only supports structs",
            ));
        }
    };
    let field_names = fields
        .named
        .iter()
        .filter_map(|f| f.ident.clone())
        .collect();
    Ok(ParsedStruct {
        name: input.ident.clone(),
        field_names,
    })
}
