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

impl FromServices for Database {
    type Error = ResolveError;

    fn from_services(services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
        let config = services.get::<Config>()?;
        Ok(Self { config })
    }
}

/// Service using derive macro, depends on [`Config`].
#[derive(FromServices)]
pub struct DerivedDatabase {
    /// Resolved configuration.
    pub config: Arc<Config>,
}

/// Async service that depends on [`Config`].
#[cfg(feature = "async")]
pub struct AsyncDatabase {
    /// Resolved configuration.
    pub config: Arc<Config>,
}

#[cfg(feature = "async")]
impl FromServicesAsync for AsyncDatabase {
    type Error = ResolveError;

    async fn from_services_async(services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
        let config = services.get::<Config>()?;
        Ok(Self { config })
    }
}

/// Async service that depends on [`AsyncDatabase`] via async resolution.
#[cfg(feature = "async")]
pub struct AsyncHandler {
    /// Resolved database.
    pub db: Arc<AsyncDatabase>,
}

#[cfg(feature = "async")]
impl FromServicesAsync for AsyncHandler {
    type Error = ResolveError;

    async fn from_services_async(services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
        let db = services.get_async::<AsyncDatabase>().await?;
        Ok(Self { db })
    }
}

/// Async service using derive macro, depends on [`Config`].
#[cfg(feature = "async")]
#[derive(FromServicesAsync)]
pub struct DerivedAsyncDatabase {
    /// Resolved configuration.
    pub config: Arc<Config>,
}

/// Stateless unit struct service for testing.
#[derive(FromServices)]
pub struct UnitService;

/// Service with mixed resolved and default fields for testing.
#[derive(FromServices)]
pub struct MixedService {
    /// Resolved configuration.
    pub config: Arc<Config>,
    /// Default port.
    #[di(default)]
    pub port: u16,
}

/// Async unit struct service for testing.
#[cfg(feature = "async")]
#[derive(FromServicesAsync)]
pub struct AsyncUnitService;
