use crate::prelude::*;

#[macro_export]
macro_rules! define_commands_web {
    ($($kind:ident($req:ty)),* $(,)?) => {
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub enum CommandRequest {
            $(
                $kind($req),
            )*
        }

        impl IRequest for CommandRequest {}

        $(
            impl From<$req> for CommandRequest {
                fn from(request: $req) -> Self {
                    Self::$kind(request)
                }
            }
        )*

        #[derive(Debug)]
        pub enum CommandResult {
            $(
                $kind($req, Result<<$req as Executable>::Response, Report<<$req as Executable>::ExecutionError>>),
            )*
        }

        impl IResult for CommandResult {}

        pub struct CommandInfo;

        impl ICommandInfo for CommandInfo {
            type Request = CommandRequest;
            #[cfg(feature = "server")]
            type Command =  Command;
            #[cfg(feature = "server")]
            type Handler = CommandHandler;
            type Result = CommandResult;
        }
    };
}

pub trait IRequest: Clone + Debug + Eq + Hash + PartialEq + Send + Sync {}

pub trait IResult: Debug + Send + Sync {}

pub trait ICommandInfo {
    type Request: IRequest;
    #[cfg(feature = "server")]
    type Command: ICommand<Self::Handler, Self::Result>;
    #[cfg(feature = "server")]
    type Handler: IHandler;
    type Result: IResult;
}
