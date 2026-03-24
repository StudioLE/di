//! Async service construction trait.
use crate::prelude::*;

/// Create an instance asynchronously by resolving dependencies from the [`ServiceProvider`].
pub trait FromProviderAsync: Send + Sync + 'static {
    /// Create an instance asynchronously by resolving dependencies from the [`ServiceProvider`].
    fn from_provider_async(
        services: &ServiceProvider,
    ) -> impl Future<Output = Result<Self, Report<ResolveError>>> + Send
    where
        Self: Sized;
}
