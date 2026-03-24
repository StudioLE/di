//! Concrete service fixtures for DI container tests.
#![allow(dead_code)]
use crate::prelude::*;

/// Minimal configuration for testing.
pub struct Config {
    /// Port number.
    pub port: u16,
}

/// Service that depends on [`Config`].
pub struct Database {
    /// Resolved configuration.
    pub config: Arc<Config>,
}

impl FromProvider for Database {
    type Error = ResolveError;

    fn from_provider(services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
        let config = services.get::<Config>()?;
        Ok(Self { config })
    }
}

/// Service using derive macro, depends on [`Config`].
#[derive(FromProvider)]
pub struct DerivedDatabase {
    /// Resolved configuration.
    pub config: Arc<Config>,
}

#[cfg(feature = "async")]
/// Async service that depends on [`Config`].
pub struct AsyncDatabase {
    /// Resolved configuration.
    pub config: Arc<Config>,
}

#[cfg(feature = "async")]
impl FromProviderAsync for AsyncDatabase {
    type Error = ResolveError;

    async fn from_provider_async(services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
        let config = services.get::<Config>()?;
        Ok(Self { config })
    }
}

#[cfg(feature = "async")]
/// Async service that depends on [`AsyncDatabase`] via async resolution.
pub struct AsyncHandler {
    /// Resolved database.
    pub db: Arc<AsyncDatabase>,
}

#[cfg(feature = "async")]
impl FromProviderAsync for AsyncHandler {
    type Error = ResolveError;

    async fn from_provider_async(services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
        let db = services.get_async::<AsyncDatabase>().await?;
        Ok(Self { db })
    }
}

#[cfg(feature = "async")]
/// Async service using derive macro, depends on [`Config`].
#[derive(FromProviderAsync)]
pub struct DerivedAsyncDatabase {
    /// Resolved configuration.
    pub config: Arc<Config>,
}
