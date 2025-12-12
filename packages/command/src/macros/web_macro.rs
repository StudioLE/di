use crate::prelude::*;

#[macro_export]
macro_rules! define_commands_web {
    ($($kind:ident($req:ty)),* $(,)?) => {
        #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
        pub enum CommandSuccess {
            $(
                $kind(<$req as Executable>::Response),
            )*
        }

        impl ISuccess for CommandSuccess {}

        #[derive(Debug)]
        pub enum CommandFailure {
            $(
                $kind(<$req as Executable>::ExecutionError),
            )*
        }

        impl IFailure for CommandFailure {}

        #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
        pub struct CommandEvent {
            pub request: CommandRequest,
            pub kind: EventKind,
        }

        impl IEvent<CommandRequest> for CommandEvent {
            fn new(request: CommandRequest, kind: EventKind) -> Self {
                Self { request, kind }
            }

            fn get_request(&self) -> &CommandRequest {
                &self.request
            }

            fn get_kind(&self) -> &EventKind {
                &self.kind
            }
        }

        pub struct CommandInfo;

        impl ICommandInfo for CommandInfo {
            type Request = CommandRequest;
            #[cfg(feature = "server")]
            type Command =  Command;
            #[cfg(feature = "server")]
            type Handler = CommandHandler;
            type Success = CommandSuccess;
            type Failure = CommandFailure;
            type Event = CommandEvent;
        }
    };
}

pub trait IRequest: Clone + Debug + Eq + Hash + PartialEq + Send + Sync {}

pub trait ISuccess: Debug + Send + Sync {}
pub trait IFailure: Debug + Send + Sync {}
pub trait IEvent<Req: IRequest>: Clone + Debug + Send + Sync {
    fn new(request: Req, kind: EventKind) -> Self;

    fn get_request(&self) -> &Req;

    fn get_kind(&self) -> &EventKind;
}

pub trait ICommandInfo {
    type Request: IRequest;
    #[cfg(feature = "server")]
    type Command: ICommand<Self::Handler, Self::Success, Self::Failure>;
    #[cfg(feature = "server")]
    type Handler: IHandler;
    type Success: ISuccess;
    type Failure: IFailure;
    type Event: IEvent<Self::Request>;
}
