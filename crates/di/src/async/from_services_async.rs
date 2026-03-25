//! Async service construction trait.
use crate::prelude::*;

/// Create an instance asynchronously by resolving dependencies from the [`ServiceProvider`].
pub trait FromServicesAsync: Send + Sync + 'static {
    /// Error type returned by [`FromServicesAsync::from_services_async`].
    type Error: StdError + Send + Sync + 'static;

    /// Create an instance asynchronously by resolving dependencies from the [`ServiceProvider`].
    fn from_services_async(
        services: &ServiceProvider,
    ) -> impl Future<Output = Result<Self, Report<Self::Error>>> + Send
    where
        Self: Sized;
}
