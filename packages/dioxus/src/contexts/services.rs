use crate::prelude::*;
use dioxus_fullstack::Lazy;

pub static SERVICES: Lazy<ServiceProvider> = Lazy::new(|| async move {
    let services = match ServiceProvider::create().await {
        Ok(services) => services,
        Err(error) => {
            error!("{error:?}");
            return Err(ServerFnError::new(error.to_string()));
        }
    };
    Ok(services)
});
