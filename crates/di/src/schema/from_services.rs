//! Service construction trait.
use crate::prelude::*;

/// Create an instance by resolving dependencies from the [`ServiceProvider`].
pub trait FromServices: Send + Sync + 'static {
    /// Error type returned by [`FromServices::from_services`].
    type Error: StdError + Send + Sync + 'static;

    /// Create an instance by resolving dependencies from the [`ServiceProvider`].
    fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>>
    where
        Self: Sized;
}
