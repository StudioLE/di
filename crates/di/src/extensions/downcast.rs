//! Downcast extension for type-erased `Arc`.
use crate::prelude::*;

/// Downcast a type-erased `Arc` with a panic on failure.
pub(crate) trait ExpectDowncast {
    /// Downcast to a concrete type, panicking if the type does not match.
    fn expect_downcast<T: Send + Sync + 'static>(self) -> Arc<T>;
}

impl ExpectDowncast for Arc<dyn Any + Send + Sync> {
    #[expect(
        clippy::panic,
        reason = "panic on downcast failure indicates internal bug"
    )]
    fn expect_downcast<T: Send + Sync + 'static>(self) -> Arc<T> {
        let type_name = type_name::<T>();
        self.downcast::<T>()
            .unwrap_or_else(|_| panic!("should be able to downcast to {type_name}"))
    }
}
