//! Async service resolution.
use crate::prelude::*;

impl ServiceProvider {
    /// Resolve a concrete type, supporting async registrations.
    pub async fn get_async<T: Send + Sync + 'static>(
        &self,
    ) -> Result<Arc<T>, Report<ResolveError>> {
        let type_name = type_name::<T>();
        trace!(type_name, "Resolving service async");
        let type_id = TypeId::of::<T>();
        if let Some(dynamic) = self.get_cached(type_id) {
            return Ok(dynamic.expect_downcast::<T>());
        }
        let registration = self.get_registration(type_id, type_name)?;
        let dynamic = self.call_factory(registration).await?;
        self.cache_if_singleton(type_id, registration.scope, &dynamic);
        Ok(dynamic.expect_downcast::<T>())
    }

    /// Call the appropriate factory for a registration, dispatching to async if needed.
    pub(super) async fn call_factory(
        &self,
        registration: &Registration,
    ) -> Result<Arc<dyn Any + Send + Sync>, Report<ResolveError>> {
        if registration.is_async {
            let async_factory = registration
                .async_factory
                .as_ref()
                .expect("is_async registration should have async_factory");
            async_factory(self.clone()).await
        } else {
            (registration.factory)(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_async_resolves_sync_registration() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_instance(Config { port: 8080 })
            .with_type::<Database>()
            .build();

        // Act
        let db = services
            .get_async::<Database>()
            .await
            .expect("should resolve");

        // Assert
        assert_eq!(db.config.port, 8080);
    }

    #[tokio::test]
    async fn get_async_resolves_async_registration() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_instance(Config { port: 9090 })
            .with_type_async::<AsyncDatabase>()
            .build();

        // Act
        let db = services
            .get_async::<AsyncDatabase>()
            .await
            .expect("should resolve");

        // Assert
        assert_eq!(db.config.port, 9090);
    }

    #[tokio::test]
    async fn async_singleton_returns_same_arc() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_instance(Config { port: 8080 })
            .with_type_async::<AsyncDatabase>()
            .build();

        // Act
        let first = services
            .get_async::<AsyncDatabase>()
            .await
            .expect("should resolve");
        let second = services
            .get_async::<AsyncDatabase>()
            .await
            .expect("should resolve");

        // Assert
        assert!(Arc::ptr_eq(&first, &second));
    }

    #[test]
    fn sync_get_on_async_registration_returns_error() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_type_async::<AsyncDatabase>()
            .build();

        // Act
        let result = services.get::<AsyncDatabase>();

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn async_service_with_async_dependency() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_instance(Config { port: 5050 })
            .with_type_async::<AsyncDatabase>()
            .with_type_async_transient::<AsyncHandler>()
            .build();

        // Act
        let handler = services
            .get_async::<AsyncHandler>()
            .await
            .expect("should resolve");

        // Assert
        assert_eq!(handler.db.config.port, 5050);
    }

    #[cfg(feature = "traits")]
    #[tokio::test]
    async fn with_trait_async_resolves_trait() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_trait_async::<dyn Get, AsyncCache>()
            .build();

        // Act
        let cache = services
            .get_trait_async::<dyn Get>()
            .await
            .expect("should resolve");

        // Assert
        assert_eq!(cache.get("missing"), None);
    }
}
