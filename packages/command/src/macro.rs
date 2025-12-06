use crate::prelude::*;

#[macro_export]
macro_rules! define_commands {
    ($($kind:ident($req:ty)),* $(,)?) => {
        #[derive(Debug)]
        pub enum CommandRequest {
            $(
                $kind($req),
            )*
        }

        impl IRequest for CommandRequest {}

        #[derive(Clone)]
        pub enum CommandHandler {
            $(
                $kind(Arc<<$req as Executable>::Handler>),
            )*
        }

        impl IHandler for CommandHandler {}

        pub enum Command {
            $(
                $kind($req, Arc<<$req as Executable>::Handler>),
            )*
        }

        #[async_trait]
        impl ICommand<CommandHandler, CommandResult> for Command {
            fn new<T: Executable + Send + Sync + 'static>(
                request: T,
                handler: CommandHandler,
            ) -> Self {
                let request_any: Box<dyn Any> = Box::new(request);
                match handler {
                    $(
                    CommandHandler::$kind(handler) => {
                        let request = request_any
                            .downcast::<$req>()
                            .expect("Request type should match handler");
                        Self::$kind(*request, handler)
                    },
                    )*
                }
            }

            async fn execute(self) -> CommandResult {
                match self {
                    $(
                        Self::$kind(request, handler) => {
                            let result = handler.execute(&request).await;
                            CommandResult::$kind(request, result)
                        },
                    )*
                }
            }
        }

        impl Display for Command {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                let name = match &self {
                    $(
                        Self::$kind(request, _) => request.to_string(),
                    )*
                };
                f.write_str(&name)
            }
        }

        #[derive(Debug)]
        pub enum CommandResult {
            $(
                $kind($req, Result<<$req as Executable>::Response, Report<<$req as Executable>::ExecutionError>>),
            )*
        }

        impl IResult for CommandResult {}

        $(
            impl From<Arc<<$req as Executable>::Handler>> for CommandHandler {
                fn from(handler: Arc<<$req as Executable>::Handler>) -> Self {
                    Self::$kind(handler)
                }
            }
        )*

        pub struct CommandInfo;

        impl ICommandInfo for CommandInfo {
            type Request = CommandRequest;
            type Command =  Command;
            type Handler = CommandHandler;
            type Result = CommandResult;
        }

        pub trait WithCommands: Sized {
            fn with_commands(self) -> impl Future<Output = Result<Self, Report<ServiceError>>> + Send;
        }

        impl WithCommands for ServiceProvider {
            async fn with_commands(mut self) -> Result<Self, Report<ServiceError>> {
                let mut registry: CommandRegistry<CommandInfo> = CommandRegistry::new();
                $(
                    let handler = self.get_service::<<$req as Executable>::Handler>().await?;
                    registry.register::<$req>(handler);
                )*
                self.add_instance(registry);
                Ok(self)
            }
        }
    };
}

pub trait IRequest: Debug + Send + Sync {}

pub trait IHandler: Clone + Send + Sync {}

pub trait IResult: Debug + Send + Sync {}

#[async_trait]
pub trait ICommand<H: IHandler, R: IResult>: Display + Send + Sync {
    fn new<T: Executable + Send + Sync + 'static>(request: T, handler: H) -> Self;
    async fn execute(self) -> R;
}

pub trait ICommandInfo {
    type Request: IRequest;
    type Command: ICommand<Self::Handler, Self::Result>;
    type Handler: IHandler;
    type Result: IResult;
}

#[derive(Debug, Error)]
#[error("Command return an error result")]
pub struct ExecutionFailed;
