use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// Derive macro for implementing the `Service` trait.
///
/// Generates a `Service` implementation that resolves all fields via
/// `ServiceProvider::get_service()`.
///
/// # Example
///
/// ```ignore
/// #[derive(Service)]
/// pub struct MyHandler {
///     http: Arc<HttpClient>,
///     metadata: Arc<MetadataRepository>,
/// }
/// ```
///
/// Generates:
///
/// ```ignore
/// impl Service for MyHandler {
///     type Error = ServiceError;
///
///     async fn from_services(
///         services: &ServiceProvider,
///     ) -> Result<Self, Report<Self::Error>> {
///         Ok(Self {
///             http: services.get_service().await?,
///             metadata: services.get_service().await?,
///         })
///     }
/// }
/// ```
#[proc_macro_derive(Service)]
pub fn derive_service(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields,
            _ => {
                return syn::Error::new_spanned(
                    &input,
                    "Service derive only supports structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(&input, "Service derive only supports structs")
                .to_compile_error()
                .into();
        }
    };

    let field_names: Vec<_> = fields
        .named
        .iter()
        .filter_map(|f| f.ident.as_ref())
        .collect();

    let expanded = quote! {
        impl Service for #name {
            type Error = ServiceError;

            async fn from_services(
                services: &ServiceProvider,
            ) -> Result<Self, Report<Self::Error>> {
                Ok(Self {
                    #(#field_names: services.get_service().await?,)*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
