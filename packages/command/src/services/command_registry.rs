use crate::prelude::*;

pub struct CommandRegistry<T: ICommandInfo> {
    handlers: HashMap<TypeId, T::Handler>,
}

impl<T: ICommandInfo> CommandRegistry<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            handlers: HashMap::default(),
        }
    }

    #[allow(clippy::as_conversions)]
    pub fn register<R: Executable + Send + Sync + 'static>(&mut self, handler: Arc<R::Handler>)
    where
        Arc<R::Handler>: Into<T::Handler>,
    {
        let request_type = TypeId::of::<R>();
        self.handlers.insert(request_type, handler.into());
    }

    /// Add a command request to the queue
    #[allow(clippy::as_conversions)]
    pub fn resolve<R: Executable + Send + Sync + 'static>(
        &self,
        request: R,
    ) -> Result<T::Command, Report<QueueError>> {
        let request_type = TypeId::of::<R>();
        let handler = self
            .handlers
            .get(&request_type)
            .ok_or(QueueError::NoMatch)
            .attach_with(|| format!("Request type: {}\n Request: {request}", type_name::<R>()))?
            .clone();
        let command = T::Command::new(request, handler);
        Ok(command)
    }
}

impl<T: ICommandInfo + 'static> Service for CommandRegistry<T> {
    type Error = ServiceError;

    async fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        let registry = CommandRegistry::new();
        Ok(registry)
    }
}

#[derive(Debug, Error)]
pub enum QueueError {
    #[error("Unable to match request to command")]
    NoMatch,
    #[error("Unable to match request to command")]
    IncorrectCommandType,
}
