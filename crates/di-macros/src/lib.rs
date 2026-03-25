//! Derive macros for `studiole-di`.
mod generate;
mod parse;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Derive [`FromServices`] for a struct with `Arc<T>` fields.
///
/// Generates a sync implementation that resolves each field
/// from the [`ServiceProvider`].
///
/// # Example
///
/// ```ignore
/// #[derive(FromServices)]
/// pub struct Database {
///     config: Arc<Config>,
/// }
/// ```
#[proc_macro_derive(FromServices, attributes(di))]
pub fn derive_from_services(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    match parse::parse_struct(&input) {
        Ok(parsed) => generate::generate_sync(&parsed).into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derive [`FromServicesAsync`] for a struct with `Arc<T>` fields.
///
/// Generates an async implementation that resolves each field
/// from the [`ServiceProvider`].
///
/// # Example
///
/// ```ignore
/// #[derive(FromServicesAsync)]
/// pub struct AsyncDatabase {
///     config: Arc<Config>,
/// }
/// ```
#[proc_macro_derive(FromServicesAsync, attributes(di))]
pub fn derive_from_services_async(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    match parse::parse_struct(&input) {
        Ok(parsed) => generate::generate_async(&parsed).into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[cfg(test)]
mod tests;
