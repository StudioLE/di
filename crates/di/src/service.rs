use crate::prelude::*;

/// Construct a service from a [`ServiceProvider`].
pub trait Service: Any + Send + Sized + Sync {
    /// Error type returned by [`Service::from_services`].
    type Error: Error + Send + Sync + 'static;

    /// Create a new instance by resolving dependencies from the [`ServiceProvider`].
    fn from_services(
        services: &ServiceProvider,
    ) -> impl Future<Output = Result<Self, Report<Self::Error>>> + Send;
}
