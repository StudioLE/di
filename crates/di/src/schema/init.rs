//! Service initialization trait.
use crate::prelude::*;

/// Perform one-time initialization after construction.
pub trait Init: Send + Sync + 'static {
    /// Initialize the service.
    fn init(&self, services: &ServiceProvider) -> Result<(), Report<InitError>>;
}

/// Errors returned by [`Init`] implementations.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum InitError {
    /// The initialization step failed.
    #[error("Service initialization failed")]
    Init,
    /// [`ServiceProvider::init`] was called more than once.
    #[error("Services already initialized")]
    AlreadyInitialized,
}
