//! Service registration entry.
use crate::prelude::*;

/// Internal registration entry pairing a scope with a factory.
pub(crate) struct Registration {
    /// Caching strategy for this service.
    pub scope: Scope,
    /// Whether the service requires async resolution.
    #[cfg_attr(
        not(feature = "async"),
        expect(dead_code, reason = "used when async feature is enabled")
    )]
    pub is_async: bool,
    /// Sync factory closure that constructs the service.
    pub factory: SyncFactory,
    /// Async factory closure, present only for async registrations.
    #[cfg_attr(
        not(feature = "async"),
        expect(dead_code, reason = "used when async feature is enabled")
    )]
    pub async_factory: Option<AsyncFactory>,
}
