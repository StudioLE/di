//! Service registration entry.
use crate::prelude::*;

/// Internal registration entry pairing a scope with a factory.
pub(crate) struct Registration {
    /// Caching strategy for this service.
    pub scope: Scope,
    /// Whether the service requires async resolution.
    pub is_async: bool,
    /// Sync factory closure that constructs the service.
    pub factory: SyncFactory,
    /// Async factory closure, present only for async registrations.
    pub async_factory: Option<AsyncFactory>,
}
