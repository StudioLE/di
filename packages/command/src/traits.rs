use crate::prelude::*;

#[async_trait]
pub trait Execute<In, Out, E> {
    async fn execute(&self, request: &In) -> Result<Out, Report<E>>;
}

pub trait Executable: Display + Sized {
    type Response: Debug + Send + Sync;
    type ExecutionError: Error + Send + Sync;
    type Handler: Execute<Self, Self::Response, Self::ExecutionError> + Send + Sync;
}
