//! Dependency injection container.
#![cfg_attr(feature = "traits", feature(unsize))]

#[cfg(feature = "async")]
mod r#async;
mod extensions;
pub mod prelude;
mod schema;
mod service_builder;
mod service_provider;
mod traits;
