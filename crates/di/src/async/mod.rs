//! Async service registration and resolution.
mod from_provider_async;
mod service_builder_async;
#[cfg(feature = "traits")]
mod service_builder_async_traits;
mod service_provider_async;
#[cfg(feature = "traits")]
mod service_provider_async_traits;

pub use from_provider_async::*;
