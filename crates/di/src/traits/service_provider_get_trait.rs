//! Trait object resolution.
use crate::prelude::*;

impl ServiceProvider {
    /// Resolve a trait object.
    pub fn get_trait<Trait: ?Sized + Send + Sync + 'static>(
        &self,
    ) -> Result<Arc<Trait>, Report<ResolveError>> {
        let type_name = type_name::<Arc<Trait>>();
        trace!(type_name, "Resolving trait service");
        let type_id = TypeId::of::<Arc<Trait>>();

        if let Some(dynamic) = self.get_cached(type_id) {
            return Ok(Arc::clone(&dynamic.expect_downcast::<Arc<Trait>>()));
        }

        let registration = self.get_registration(type_id, type_name)?;
        #[cfg(feature = "async")]
        if registration.is_async {
            return Err(Report::new(ResolveError::Async)).attach("type", type_name);
        }
        let dynamic = (registration.factory)(self)?;
        self.cache_if_singleton(type_id, registration.scope, &dynamic);
        Ok(Arc::clone(&dynamic.expect_downcast::<Arc<Trait>>()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unregistered_type_returns_not_found() {
        // Arrange
        let services = ServiceBuilder::new().build();

        // Act
        let result = services.get_trait::<dyn Get>();

        // Assert
        assert!(result.is_err());
    }
}
