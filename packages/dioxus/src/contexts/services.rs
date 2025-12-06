use crate::prelude::*;
use dioxus_fullstack::Lazy;

pub static SERVICES: Lazy<Arc<ServiceProvider>> =
    Lazy::new::<_, _, Infallible>(|| async move { Ok(Arc::new(ServiceProvider::new())) });

pub static METADATA: Lazy<Arc<MetadataRepository>> = Lazy::new(|| async move {
    SERVICES
        .get_service::<MetadataRepository>()
        .await
        .map_err(|error| {
            error!("{error:?}");
            ServerFnError::new(error.to_string())
        })
});
