//! Clean trait object registration via `Unsize` coercion.
use crate::prelude::*;

impl ServiceBuilder {
    /// Register a concrete type as a singleton and resolve it as a trait object.
    ///
    /// - Automatically registers `Impl` via [`with_type`](ServiceBuilder::with_type)
    ///   if it has not already been registered
    /// - If `Impl` was already registered (by a prior `with_trait` or `with_type`
    ///   call), the existing registration and its scope are kept unchanged
    ///
    /// # Example
    ///
    /// ```ignore
    /// ServiceBuilder::new()
    ///     .with_trait::<dyn Get, MemoryCache>()
    ///     .with_trait::<dyn Set, MemoryCache>()
    ///     .build();
    /// ```
    #[must_use]
    pub fn with_trait<Trait: ?Sized + Send + Sync + 'static, Impl>(self) -> Self
    where
        Impl: FromServices + Unsize<Trait>,
    {
        self.register_trait::<Trait, Impl>(Scope::Singleton)
    }

    /// Register a concrete type as transient and resolve it as a trait object.
    ///
    /// - Automatically registers `Impl` via [`with_type_transient`](ServiceBuilder::with_type_transient)
    ///   if it has not already been registered
    /// - If `Impl` was already registered (by a prior `with_trait_transient` or `with_type_transient`
    ///   call), the existing registration and its scope are kept unchanged
    #[must_use]
    pub fn with_trait_transient<Trait: ?Sized + Send + Sync + 'static, Impl>(self) -> Self
    where
        Impl: FromServices + Unsize<Trait>,
    {
        self.register_trait::<Trait, Impl>(Scope::Transient)
    }

    /// Register a trait object with the given scope.
    #[expect(
        clippy::as_conversions,
        reason = "unsizing coercion to resolve concrete type as trait object"
    )]
    fn register_trait<Trait: ?Sized + Send + Sync + 'static, Impl>(mut self, scope: Scope) -> Self
    where
        Impl: FromServices + Unsize<Trait>,
    {
        if !self.factories.contains_key(&TypeId::of::<Impl>()) {
            self = self.register_type::<Impl>(scope);
        }
        let type_id = TypeId::of::<Arc<Trait>>();
        let factory: SyncFactory = Box::new(|services: &ServiceProvider| {
            let instance = services.get::<Impl>()? as Arc<Trait>;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_trait_resolves_trait() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_trait::<dyn Get, MemoryCache>()
            .build();

        // Act
        let cache = services.get_trait::<dyn Get>().expect("should resolve");

        // Assert
        assert_eq!(cache.get("missing"), None);
    }

    #[test]
    fn with_trait_multiple_traits_share_concrete_singleton() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_trait::<dyn Get, MemoryCache>()
            .with_trait::<dyn Set, MemoryCache>()
            .build();

        // Act
        let setter = services.get_trait::<dyn Set>().expect("should resolve");
        setter.set("key", "value");
        let getter = services.get_trait::<dyn Get>().expect("should resolve");

        // Assert - both traits resolve through the same concrete singleton
        assert_eq!(getter.get("key"), Some(String::from("value")));
    }

    #[test]
    fn with_trait_mock_swapped() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_trait::<dyn Get, MockCache>()
            .build();

        // Act
        let cache = services.get_trait::<dyn Get>().expect("should resolve");

        // Assert
        assert_eq!(cache.get("key"), None);
    }

    #[test]
    fn with_trait_shared_singleton() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_trait::<dyn Get, MemoryCache>()
            .with_trait::<dyn Set, MemoryCache>()
            .build();

        // Act
        let first = services.get::<MemoryCache>().expect("should resolve");
        first.set("key", "value");
        let second = services.get::<MemoryCache>().expect("should resolve");

        // Assert
        assert_eq!(second.get("key"), Some(String::from("value")));
    }
}
