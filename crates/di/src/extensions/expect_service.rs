#![expect(clippy::expect_fun_call)]
//! Panicking service resolution.
//!
//! - Async expect methods will report the callee location as `#[track_caller]` is unstable
//!   for async fn (rust#110011).
use crate::prelude::*;

impl ServiceProvider {
    /// Resolve a concrete type or panic.
    #[must_use]
    #[track_caller]
    pub fn expect<T: Send + Sync + 'static>(&self) -> Arc<T> {
        self.get::<T>().expect(&message::<T>())
    }

    /// Resolve a concrete type asynchronously or panic.
    #[cfg(feature = "async")]
    pub async fn expect_async<T: Send + Sync + 'static>(&self) -> Arc<T> {
        self.get_async::<T>().await.expect(&message::<T>())
    }

    /// Resolve a trait object or panic.
    #[cfg(feature = "traits")]
    #[must_use]
    #[track_caller]
    pub fn expect_trait<T: ?Sized + Send + Sync + 'static>(&self) -> Arc<T> {
        self.get_trait::<T>().expect(&message::<T>())
    }

    /// Resolve a trait object asynchronously or panic.
    #[cfg(all(feature = "traits", feature = "async"))]
    pub async fn expect_trait_async<T: ?Sized + Send + Sync + 'static>(&self) -> Arc<T> {
        self.get_trait_async::<T>().await.expect(&message::<T>())
    }
}

fn message<T: ?Sized>() -> String {
    format!("should be able to resolve: {}", type_name::<T>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_provider_expect() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_instance(Config { port: 8080 })
            .build();
        // Act
        let config = services.expect::<Config>();
        // Assert
        assert_eq!(config.port, 8080);
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn service_provider_expect_async() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_instance(Config { port: 8080 })
            .build();
        // Act
        let config = services.expect_async::<Config>().await;
        // Assert
        assert_eq!(config.port, 8080);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn service_provider_expect_trait() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_trait::<dyn Get, MemoryCache>()
            .build();
        // Act
        let cache = services.expect_trait::<dyn Get>();
        // Assert
        assert_eq!(cache.get("missing"), None);
    }

    #[cfg(all(feature = "traits", feature = "async"))]
    #[tokio::test]
    async fn service_provider_expect_trait_async() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_trait_async::<dyn Get, AsyncCache>()
            .build();
        // Act
        let cache = services.expect_trait_async::<dyn Get>().await;
        // Assert
        assert_eq!(cache.get("missing"), None);
    }
}
