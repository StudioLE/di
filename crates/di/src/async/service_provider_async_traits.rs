//! Async trait object resolution.
use crate::prelude::*;

impl ServiceProvider {
    /// Resolve a trait object, supporting async registrations.
    pub async fn get_trait_async<Trait: ?Sized + Send + Sync + 'static>(
        &self,
    ) -> Result<Arc<Trait>, Report<ResolveError>> {
        let type_name = type_name::<Arc<Trait>>();
        trace!(type_name, "Resolving trait service async");
        let type_id = TypeId::of::<Arc<Trait>>();
        if let Some(dynamic) = self.get_cached(type_id) {
            return Ok(Arc::clone(&dynamic.expect_downcast::<Arc<Trait>>()));
        }
        let registration = self.get_registration(type_id, type_name)?;
        let dynamic = self.call_factory(registration).await?;
        self.cache_if_singleton(type_id, registration.scope, &dynamic);
        Ok(Arc::clone(&dynamic.expect_downcast::<Arc<Trait>>()))
    }
}
