use crate::prelude::*;

#[macro_export]
macro_rules! define_commands_server {
    ($($kind:ident($req:ty, $handler:ty)),* $(,)?) => {
        #[derive(Clone)]
        pub enum CommandHandler {
            $(
                $kind(Arc<$handler>),
            )*
        }

        impl IHandler for CommandHandler {}

        pub enum Command {
            $(
                $kind($req, Arc<$handler>),
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

        $(
            impl From<Arc<$handler>> for CommandHandler {
                fn from(handler: Arc<$handler>) -> Self {
                    Self::$kind(handler)
                }
            }
        )*

        pub trait WithCommands: Sized {
            fn with_commands(self) -> impl Future<Output = Result<Self, Report<ServiceError>>> + Send;
        }

        impl WithCommands for ServiceProvider {
            async fn with_commands(mut self) -> Result<Self, Report<ServiceError>> {
                let mut registry: CommandRegistry<CommandInfo> = CommandRegistry::new();
                $(
                    let handler = self.get_service::<$handler>().await?;
                    registry.register::<$req, $handler>(handler);
                )*
                self.add_instance(registry);
                Ok(self)
            }
        }
    };
}

pub trait IHandler: Clone + Send + Sync {}

#[async_trait]
pub trait ICommand<H: IHandler, R: IResult>: Display + Send + Sync {
    fn new<T: Executable + Send + Sync + 'static>(request: T, handler: H) -> Self;
    async fn execute(self) -> R;
}
