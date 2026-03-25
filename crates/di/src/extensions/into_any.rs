//! Type-erase a value into a dynamic `Arc`.
use crate::prelude::*;

/// Convert a concrete value into a type-erased `Arc`.
pub(crate) trait IntoAny: Send + Sync + 'static {
    /// Wrap in an `Arc` and erase the concrete type.
    fn into_any(self) -> Arc<dyn Any + Send + Sync>;
}

impl<T: Send + Sync + 'static> IntoAny for T {
    #[expect(
        clippy::as_conversions,
        reason = "upcast to trait object for type-erased storage"
    )]
    fn into_any(self) -> Arc<dyn Any + Send + Sync> {
        Arc::new(self) as Arc<dyn Any + Send + Sync>
    }
}
