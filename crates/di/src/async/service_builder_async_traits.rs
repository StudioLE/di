//! Async trait service registration.
use crate::prelude::*;

impl ServiceBuilder {
    /// Register an async concrete type as a singleton and resolve it as a trait object.
    ///
    /// - Automatically registers `Impl` via [`with_type_async`](ServiceBuilder::with_type_async)
    ///   if it has not already been registered
    /// - If `Impl` was already registered (by a prior `with_trait_async` or `with_type_async`
    ///   call), the existing registration and its scope are kept unchanged
    ///
    /// # Example
    ///
    /// ```ignore
    /// ServiceBuilder::new()
    ///     .with_trait_async::<dyn Get, AsyncCache>()
    ///     .with_trait_async::<dyn Set, AsyncCache>()
    ///     .build();
    /// ```
    #[must_use]
    pub fn with_trait_async<Trait: ?Sized + Send + Sync + 'static, Impl>(self) -> Self
    where
        Impl: FromProviderAsync + Unsize<Trait>,
    {
        self.register_trait_async::<Trait, Impl>(Scope::Singleton)
    }

    /// Register an async concrete type as transient and resolve it as a trait object.
    ///
    /// - Automatically registers `Impl` via [`with_type_async_transient`](ServiceBuilder::with_type_async_transient)
    ///   if it has not already been registered
    /// - If `Impl` was already registered (by a prior `with_trait_async_transient` or
    ///   `with_type_async_transient` call), the existing registration and its scope are
    ///   kept unchanged
    #[must_use]
    pub fn with_trait_async_transient<Trait: ?Sized + Send + Sync + 'static, Impl>(self) -> Self
    where
        Impl: FromProviderAsync + Unsize<Trait>,
    {
        self.register_trait_async::<Trait, Impl>(Scope::Transient)
    }

    /// Register an async trait object with the given scope.
    #[expect(
        clippy::as_conversions,
        reason = "unsizing coercion to resolve concrete type as trait object"
    )]
    fn register_trait_async<Trait: ?Sized + Send + Sync + 'static, Impl>(
        mut self,
        scope: Scope,
    ) -> Self
    where
        Impl: FromProviderAsync + Unsize<Trait>,
    {
        if !self.factories.contains_key(&TypeId::of::<Impl>()) {
            self = self.register_type_async::<Impl>(scope);
        }
        let type_id = TypeId::of::<Arc<Trait>>();
        let sync_factory: SyncFactory = Box::new(|_services: &ServiceProvider| {
            Err(Report::new(ResolveError::Async)).attach("type", type_name::<Arc<Trait>>())
        });
        let async_factory: AsyncFactory = Box::new(|services: ServiceProvider| {
            Box::pin(async move {
                let instance = services.get_async::<Impl>().await? as Arc<Trait>;
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
