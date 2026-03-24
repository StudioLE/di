//! Internal service registry storage.
use crate::prelude::*;

/// Internal storage for registered services.
///
/// Wrapped in an `Arc` by [`ServiceProvider`] so that async factories can
/// take ownership of a cloned provider without borrowing from the caller's stack.
pub(crate) struct ServiceRegistry {
    /// Factory registrations keyed by type.
    pub factories: HashMap<TypeId, Registration>,
    /// Cached singleton instances keyed by type.
    pub instances: Mutex<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}
