use crate::prelude::*;

pub trait Service: Any + Send + Sized + Sync {
    type Error: Error;

    fn from_services(
        services: &ServiceProvider,
    ) -> impl Future<Output = Result<Self, Report<Self::Error>>> + Send;
}
