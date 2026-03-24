//! Derive macros for `studiole-di`.
mod generate;
mod parse;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Derive [`FromProvider`] for a struct with `Arc<T>` fields.
///
/// Generates a sync implementation that resolves each field
/// from the [`ServiceProvider`].
///
/// # Example
///
/// ```ignore
/// #[derive(FromProvider)]
/// pub struct Database {
///     config: Arc<Config>,
/// }
/// ```
#[proc_macro_derive(FromProvider)]
pub fn derive_from_provider(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    match parse::parse_struct(&input) {
        Ok(parsed) => generate::generate_sync(&parsed).into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derive [`FromProviderAsync`] for a struct with `Arc<T>` fields.
///
/// Generates an async implementation that resolves each field
/// from the [`ServiceProvider`].
///
/// # Example
///
/// ```ignore
/// #[derive(FromProviderAsync)]
/// pub struct AsyncDatabase {
///     config: Arc<Config>,
/// }
/// ```
#[proc_macro_derive(FromProviderAsync)]
pub fn derive_from_provider_async(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    match parse::parse_struct(&input) {
        Ok(parsed) => generate::generate_async(&parsed).into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[cfg(test)]
mod tests;
