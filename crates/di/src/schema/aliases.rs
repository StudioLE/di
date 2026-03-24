//! Type aliases for factory closures and results.
use crate::prelude::*;

/// Type-erased result from a factory.
pub type FactoryResult = Result<Arc<dyn Any + Send + Sync>, Report<ResolveError>>;

/// Sync factory closure.
pub type SyncFactory = Box<dyn Fn(&ServiceProvider) -> FactoryResult + Send + Sync>;

/// Pinned, boxed, sendable future.
pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

/// Async factory closure.
///
/// Takes `ServiceProvider` by value rather than by reference because the
/// returned future must be `Send + 'static`, so it cannot borrow from the
/// caller's stack. Since `ServiceProvider` is a cheap `Arc` clone this has
/// negligible cost.
pub type AsyncFactory = Box<dyn Fn(ServiceProvider) -> BoxFuture<FactoryResult> + Send + Sync>;
