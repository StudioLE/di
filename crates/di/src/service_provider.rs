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

    /// Run all registered init closures.
    ///
    /// Returns [`InitError::AlreadyInitialized`] if called more than once.
    pub fn init(self) -> Result<Self, Report<InitError>> {
        if self.registry.initialized.swap(true, Ordering::SeqCst) {
            return Err(Report::new(InitError::AlreadyInitialized));
        }
        for init_fn in &self.registry.inits {
            init_fn(&self)?;
        }
        Ok(self)
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

    #[test]
    fn derived_struct_resolves() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_instance(Config { port: 8080 })
            .with_type::<DerivedDatabase>()
            .build();
        // Act
        let db = services
            .get::<DerivedDatabase>()
            .expect("DerivedDatabase should resolve");
        // Assert
        assert_eq!(db.config.port, 8080);
    }

    #[test]
    fn unit_struct_resolves() {
        // Arrange
        let services = ServiceBuilder::new().with_type::<UnitService>().build();
        // Act
        let result = services.get::<UnitService>();
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn service_provider_init() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_type::<InitTracker>()
            .with_init::<InitTracker>()
            .build();
        // Act
        let services = services.init().expect("should init");
        // Assert
        let tracker = services.get::<InitTracker>().expect("should resolve");
        assert!(tracker.initialized.load(Ordering::SeqCst));
    }

    #[test]
    fn service_provider_init_failure() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_type::<FailingInit>()
            .with_init::<FailingInit>()
            .build();
        // Act
        let output = services.init();
        // Assert
        assert!(output.is_err());
    }

    #[test]
    fn service_provider_init_already_initialized() {
        // Arrange
        let services = ServiceBuilder::new().build().init().expect("should init");
        // Act
        let output = services.clone().init();
        // Assert
        assert!(output.is_err());
    }

    #[test]
    fn service_provider_init_order() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_instance(InitOrder {
                calls: Mutex::new(Vec::new()),
            })
            .with_type::<OrderedInitA>()
            .with_init::<OrderedInitA>()
            .with_type::<OrderedInitB>()
            .with_init::<OrderedInitB>()
            .build();
        // Act
        let services = services.init().expect("should init");
        // Assert
        let order = services.get::<InitOrder>().expect("should resolve");
        let calls = order.calls.lock().expect("should lock");
        assert_eq!(*calls, vec!["A".to_owned(), "B".to_owned()]);
    }

    #[test]
    fn mixed_default_fields_resolve() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_instance(Config { port: 8080 })
            .with_type::<MixedService>()
            .build();
        // Act
        let svc = services
            .get::<MixedService>()
            .expect("MixedService should resolve");
        // Assert
        assert_eq!(svc.config.port, 8080);
        assert_eq!(svc.port, 0);
    }

    struct InitTracker {
        initialized: AtomicBool,
    }

    impl FromServices for InitTracker {
        type Error = ResolveError;
        fn from_services(_services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
            Ok(Self {
                initialized: AtomicBool::new(false),
            })
        }
    }

    impl Init for InitTracker {
        fn init(&self, _services: &ServiceProvider) -> Result<(), Report<InitError>> {
            self.initialized.store(true, Ordering::SeqCst);
            Ok(())
        }
    }

    struct FailingInit;

    impl FromServices for FailingInit {
        type Error = ResolveError;
        fn from_services(_services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
            Ok(Self)
        }
    }

    impl Init for FailingInit {
        fn init(&self, _services: &ServiceProvider) -> Result<(), Report<InitError>> {
            Err(Report::new(InitError::Init))
        }
    }

    struct InitOrder {
        calls: Mutex<Vec<String>>,
    }

    struct OrderedInitA;

    impl FromServices for OrderedInitA {
        type Error = ResolveError;
        fn from_services(_services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
            Ok(Self)
        }
    }

    impl Init for OrderedInitA {
        fn init(&self, services: &ServiceProvider) -> Result<(), Report<InitError>> {
            let order = services
                .get::<InitOrder>()
                .change_context(InitError::Init)?;
            order
                .calls
                .lock()
                .expect("should lock")
                .push("A".to_owned());
            Ok(())
        }
    }

    struct OrderedInitB;

    impl FromServices for OrderedInitB {
        type Error = ResolveError;
        fn from_services(_services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
            Ok(Self)
        }
    }

    impl Init for OrderedInitB {
        fn init(&self, services: &ServiceProvider) -> Result<(), Report<InitError>> {
            let order = services
                .get::<InitOrder>()
                .change_context(InitError::Init)?;
            order
                .calls
                .lock()
                .expect("should lock")
                .push("B".to_owned());
            Ok(())
        }
    }
}
