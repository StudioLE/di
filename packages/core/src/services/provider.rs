use crate::prelude::*;
use std::any::type_name;

pub struct ServiceProvider {
    factories:
        HashMap<TypeId, Box<dyn Fn(&ServiceProvider) -> Arc<dyn Any + Send + Sync> + Send + Sync>>,
    instances: Mutex<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl ServiceProvider {
    #[must_use]
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
            instances: Mutex::new(HashMap::new()),
        }
    }

    #[allow(clippy::as_conversions)]
    pub fn add_instance<T>(&mut self, instance: T)
    where
        T: Service + 'static,
    {
        let type_id = TypeId::of::<T>();
        let dynamic = Arc::new(instance) as Arc<dyn Any + Send + Sync>;
        self.instances
            .lock()
            .expect("should be able to lock instances")
            .insert(type_id, dynamic);
    }

    #[allow(clippy::as_conversions)]
    pub fn add_factory<T, F>(&mut self, factory: F)
    where
        T: Service + 'static,
        F: Fn(&ServiceProvider) -> T + Send + Sync + 'static,
    {
        self.factories.insert(
            TypeId::of::<T>(),
            Box::new(move |services| Arc::new(factory(services)) as Arc<dyn Any + Send + Sync>),
        );
    }

    pub async fn get_service<T: Service>(&self) -> Result<Arc<T>, Report<ServiceError>> {
        let type_name = type_name::<T>();
        trace!("Resolving required service: {type_name}");
        let type_id = TypeId::of::<T>();
        let option = self
            .get_existing_service::<T>(type_id, type_name)
            .or_else(|| self.create_registered_service::<T>(type_id, type_name));
        if let Some(service) = option {
            return Ok(service);
        }
        self.create_unregistered_service::<T>(type_id, type_name)
            .await
    }

    #[allow(clippy::panic)]
    fn get_existing_service<T: Service>(&self, type_id: TypeId, type_name: &str) -> Option<Arc<T>> {
        let instances = self
            .instances
            .lock()
            .expect("should be able to lock instances");
        let dynamic = instances.get(&type_id)?;
        let instance = Arc::clone(dynamic)
            .downcast::<T>()
            .unwrap_or_else(|_| panic!("should be able to downcast to {type_name}"));
        Some(instance)
    }

    #[allow(clippy::panic)]
    fn create_registered_service<T: Service>(
        &self,
        type_id: TypeId,
        type_name: &str,
    ) -> Option<Arc<T>> {
        let factory = self.factories.get(&type_id)?;
        let dynamic = factory(self);
        let instance = Arc::clone(&dynamic)
            .downcast::<T>()
            .unwrap_or_else(|_| panic!("should be able to downcast to {type_name}"));
        self.instances
            .lock()
            .expect("should be able to lock instances")
            .insert(type_id, dynamic);
        Some(instance)
    }

    #[allow(clippy::as_conversions)]
    async fn create_unregistered_service<T: Service>(
        &self,
        type_id: TypeId,
        type_name: &str,
    ) -> Result<Arc<T>, Report<ServiceError>> {
        let instance = T::from_services(self)
            .await
            .change_context(ServiceError::Create)
            .attach_with(|| format!("Type: {type_name}"))?;
        let instance = Arc::new(instance);
        let dynamic = Arc::clone(&instance) as Arc<dyn Any + Send + Sync>;
        self.instances
            .lock()
            .expect("should be able to lock instances")
            .insert(type_id, dynamic);
        Ok(instance)
    }
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Unable to resolve service")]
    NoService,
    #[error("Unable to create service")]
    Create,
}
