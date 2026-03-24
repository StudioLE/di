//! Service construction trait.
use crate::prelude::*;

/// Create an instance by resolving dependencies from the [`ServiceProvider`].
pub trait FromProvider: Send + Sync + 'static {
    /// Create an instance by resolving dependencies from the [`ServiceProvider`].
    fn from_provider(services: &ServiceProvider) -> Result<Self, Report<ResolveError>>
    where
        Self: Sized;
}
