//! Service registration builder.
use crate::prelude::*;

/// Build a [`ServiceProvider`] by registering services.
#[derive(Default)]
pub struct ServiceBuilder {
    /// Factory registrations keyed by type.
    pub(crate) factories: HashMap<TypeId, Registration>,
    /// Pre-built singleton instances keyed by type.
    pub(crate) instances: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl ServiceBuilder {
    /// Create an empty [`ServiceBuilder`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a pre-built instance as a singleton.
    #[must_use]
    pub fn with_instance<T: Send + Sync + 'static>(mut self, value: T) -> Self {
        let type_id = TypeId::of::<T>();
        let dynamic = value.into_any();
        self.instances.insert(type_id, dynamic);
        self
    }

    /// Register a singleton type for resolution via [`FromServices`].
    #[must_use]
    pub fn with_type<T: FromServices>(self) -> Self {
        self.register_type::<T>(Scope::Singleton)
    }

    /// Register a transient type for resolution via [`FromServices`].
    #[must_use]
    pub fn with_type_transient<T: FromServices>(self) -> Self {
        self.register_type::<T>(Scope::Transient)
    }

    /// Register a type with the given scope.
    pub(crate) fn register_type<T: FromServices>(mut self, scope: Scope) -> Self {
        let type_id = TypeId::of::<T>();
        let factory: SyncFactory = Box::new(|services: &ServiceProvider| {
            let instance = T::from_services(services)
                .change_context(ResolveError::Factory)
                .attach("type", type_name::<T>())?;
            Ok(instance.into_any())
        });
        self.factories.insert(
            type_id,
            Registration {
                scope,
                is_async: false,
                factory,
                async_factory: None,
            },
        );
        self
    }

    /// Build the [`ServiceProvider`] from the registered services.
    #[must_use]
    pub fn build(self) -> ServiceProvider {
        ServiceProvider {
            registry: Arc::new(ServiceRegistry {
                factories: self.factories,
                instances: Mutex::new(self.instances),
            }),
        }
    }
}
