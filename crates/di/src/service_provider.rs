//! Service resolution.

use crate::prelude::*;

/// Resolve registered services.
#[derive(Clone)]
pub struct ServiceProvider {
    /// Shared reference to the service registry.
    pub(crate) registry: Arc<ServiceRegistry>,
}

impl ServiceProvider {
    /// Resolve a concrete type.
    pub fn get<T: Send + Sync + 'static>(&self) -> Result<Arc<T>, Report<ResolveError>> {
        let type_name = type_name::<T>();
        trace!(type_name, "Resolving service");
        let type_id = TypeId::of::<T>();

        if let Some(dynamic) = self.get_cached(type_id) {
            return Ok(dynamic.expect_downcast::<T>());
        }

        let registration = self.get_registration(type_id, type_name)?;
        #[cfg(feature = "async")]
        if registration.is_async {
            return Err(Report::new(ResolveError::Async)).attach("type", type_name);
        }
        let dynamic = (registration.factory)(self)?;
        self.cache_if_singleton(type_id, registration.scope, &dynamic);
        Ok(dynamic.expect_downcast::<T>())
    }

    /// Look up a cached instance by type.
    pub(crate) fn get_cached(&self, type_id: TypeId) -> Option<Arc<dyn Any + Send + Sync>> {
        let instances = self
            .registry
            .instances
            .lock()
            .expect("should be able to lock instances");
        instances.get(&type_id).map(Arc::clone)
    }

    /// Look up a registration by type.
    pub(crate) fn get_registration(
        &self,
        type_id: TypeId,
        type_name: &'static str,
    ) -> Result<&Registration, Report<ResolveError>> {
        self.registry
            .factories
            .get(&type_id)
            .ok_or_else(|| Report::new(ResolveError::NotFound))
            .attach("type", type_name)
    }

    /// Cache an instance if the registration is a singleton.
    pub(crate) fn cache_if_singleton(
        &self,
        type_id: TypeId,
        scope: Scope,
        dynamic: &Arc<dyn Any + Send + Sync>,
    ) {
        if scope == Scope::Singleton {
            self.registry
                .instances
                .lock()
                .expect("should be able to lock instances")
                .insert(type_id, Arc::clone(dynamic));
        }
    }
}

/// Errors returned when resolving a service.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ResolveError {
    /// No service was registered for the requested type.
    #[error("Service not registered")]
    NotFound,
    /// The factory function failed during service construction.
    #[error("Factory failed to construct service")]
    Factory,
    /// The service requires async resolution but was called synchronously.
    #[cfg(feature = "async")]
    #[error("Service requires async resolution")]
    Async,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton_shares_state() {
        // Arrange
        let services = ServiceBuilder::new().with_type::<MemoryCache>().build();

        // Act
        let first = services.get::<MemoryCache>().expect("should resolve");
        first.set("key", "hello");
        let second = services.get::<MemoryCache>().expect("should resolve");

        // Assert
        assert_eq!(second.get("key"), Some(String::from("hello")));
    }

    #[test]
    fn transient_does_not_share_state() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_type_transient::<MemoryCache>()
            .build();

        // Act
        let first = services.get::<MemoryCache>().expect("should resolve");
        first.set("key", "hello");
        let second = services.get::<MemoryCache>().expect("should resolve");

        // Assert
        assert_eq!(second.get("key"), None);
    }

    #[test]
    fn unregistered_type_returns_not_found() {
        // Arrange
        let services = ServiceBuilder::new().build();

        // Act
        let result = services.get::<Config>();

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn resolve_instance() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_instance(Config { port: 3000 })
            .build();

        // Act
        let config = services.get::<Config>().expect("should resolve");

        // Assert
        assert_eq!(config.port, 3000);
    }

    #[test]
    fn cloned_provider_shares_singleton() {
        // Arrange
        let services = ServiceBuilder::new().with_type::<MemoryCache>().build();

        // Act
        let first = services.get::<MemoryCache>().expect("should resolve");
        first.set("key", "hello");
        let cloned = services.clone();
        let second = cloned.get::<MemoryCache>().expect("should resolve");

        // Assert
        assert_eq!(second.get("key"), Some(String::from("hello")));
    }
}
