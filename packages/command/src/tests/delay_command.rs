use crate::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DelayRequest {
    name: String,
    duration: u64,
}

impl DelayRequest {
    #[must_use]
    pub fn new(name: String, duration: u64) -> Self {
        Self { name, duration }
    }
}

impl Default for DelayRequest {
    fn default() -> Self {
        Self::new("Delay 50 ms".to_owned(), 50)
    }
}

// TODO: Remove this
//
// impl From<DelayRequest> for CommandRequestTrait {
//     fn from(request: DelayRequest) -> Self {
//         Self::Delay(request)
//     }
// }

impl Executable for DelayRequest {
    type Response = ();
    type ExecutionError = Infallible;
    type Handler = DelayHandler;
}

#[derive(Debug)]
pub struct DelayHandler;

impl Service for DelayHandler {
    type Error = Infallible;

    async fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self)
    }
}

impl Display for DelayRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.name)
    }
}

#[async_trait]
impl Execute<DelayRequest, (), Infallible> for DelayHandler {
    async fn execute(&self, request: &DelayRequest) -> Result<(), Report<Infallible>> {
        trace!(%request, "Executing {} ms delay", request.duration);
        sleep(Duration::from_millis(request.duration)).await;
        Ok(())
    }
}
