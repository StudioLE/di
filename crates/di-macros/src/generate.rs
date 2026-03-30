//! Generate trait implementations from parsed struct data.
use crate::parse::ParsedStruct;
use proc_macro2::TokenStream;
use quote::quote;

/// Generate a sync `FromServices` implementation.
pub(crate) fn generate_sync(parsed: &ParsedStruct) -> TokenStream {
    let name = &parsed.name;
    let service_fields = &parsed.service_fields;
    let default_fields = &parsed.default_fields;
    let trait_field_names = parsed.trait_fields.iter().map(|f| &f.name);
    let trait_field_types = parsed.trait_fields.iter().map(|f| &f.trait_type);
    quote! {
        impl ::studiole_di::FromServices for #name {
            type Error = ::studiole_di::ResolveError;

            fn from_services(
                services: &::studiole_di::ServiceProvider,
            ) -> ::std::result::Result<Self, ::studiole_report::prelude::Report<::studiole_di::ResolveError>> {
                Ok(Self {
                    #(#service_fields: services.get()?,)*
                    #(#trait_field_names: services.get_trait::<#trait_field_types>()?,)*
                    #(#default_fields: Default::default(),)*
                })
            }
        }
    }
}

/// Generate an async `FromServicesAsync` implementation.
pub(crate) fn generate_async(parsed: &ParsedStruct) -> TokenStream {
    let name = &parsed.name;
    let service_fields = &parsed.service_fields;
    let default_fields = &parsed.default_fields;
    let trait_field_names = parsed.trait_fields.iter().map(|f| &f.name);
    let trait_field_types = parsed.trait_fields.iter().map(|f| &f.trait_type);
    quote! {
        impl ::studiole_di::FromServicesAsync for #name {
            type Error = ::studiole_di::ResolveError;

            async fn from_services_async(
                services: &::studiole_di::ServiceProvider,
            ) -> ::std::result::Result<Self, ::studiole_report::prelude::Report<::studiole_di::ResolveError>> {
                Ok(Self {
                    #(#service_fields: services.get_async().await?,)*
                    #(#trait_field_names: services.get_trait_async::<#trait_field_types>().await?,)*
                    #(#default_fields: Default::default(),)*
                })
            }
        }
    }
}
