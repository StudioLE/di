//! Generate trait implementations from parsed struct data.
use crate::parse::ParsedStruct;
use proc_macro2::TokenStream;
use quote::quote;

/// Generate a sync `FromProvider` implementation.
pub(crate) fn generate_sync(parsed: &ParsedStruct) -> TokenStream {
    let name = &parsed.name;
    let field_names = &parsed.field_names;
    quote! {
        impl ::studiole_di::FromProvider for #name {
            fn from_provider(
                services: &::studiole_di::ServiceProvider,
            ) -> ::std::result::Result<Self, ::studiole_report::prelude::Report<::studiole_di::ResolveError>> {
                Ok(Self {
                    #(#field_names: services.get()?,)*
                })
            }
        }
    }
}

/// Generate an async `FromProviderAsync` implementation.
pub(crate) fn generate_async(parsed: &ParsedStruct) -> TokenStream {
    let name = &parsed.name;
    let field_names = &parsed.field_names;
    quote! {
        impl ::studiole_di::FromProviderAsync for #name {
            async fn from_provider_async(
                services: &::studiole_di::ServiceProvider,
            ) -> ::std::result::Result<Self, ::studiole_report::prelude::Report<::studiole_di::ResolveError>> {
                Ok(Self {
                    #(#field_names: services.get_async().await?,)*
                })
            }
        }
    }
}
