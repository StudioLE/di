//! Async service registration.
use crate::prelude::*;

impl ServiceBuilder {
    /// Register a singleton type for async resolution via [`FromProviderAsync`].
    #[must_use]
    pub fn with_type_async<T: FromProviderAsync>(self) -> Self {
        self.register_type_async::<T>(Scope::Singleton)
    }

    /// Register a transient type for async resolution via [`FromProviderAsync`].
    #[must_use]
    pub fn with_type_async_transient<T: FromProviderAsync>(self) -> Self {
        self.register_type_async::<T>(Scope::Transient)
    }

    /// Register an async type with the given scope.
    pub(super) fn register_type_async<T: FromProviderAsync>(mut self, scope: Scope) -> Self {
        let type_id = TypeId::of::<T>();
        let sync_factory: SyncFactory = Box::new(|_services: &ServiceProvider| {
            Err(Report::new(ResolveError::Async)).attach("type", type_name::<T>())
        });
        let async_factory: AsyncFactory = Box::new(|services: ServiceProvider| {
            Box::pin(async move {
                let instance = T::from_provider_async(&services)
                    .await
                    .change_context(ResolveError::Factory)
                    .attach("type", type_name::<T>())?;
                Ok(instance.into_any())
            })
        });
        self.factories.insert(
            type_id,
            Registration {
                scope,
                is_async: true,
                factory: sync_factory,
                async_factory: Some(async_factory),
            },
        );
        self
    }
}
