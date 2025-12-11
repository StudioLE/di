use crate::prelude::*;

pub enum CommandStatus<T: ICommandInfo> {
    Queued(T::Command),
    Executing,
    Completed(T::Result),
}

impl<T: ICommandInfo> Debug for CommandStatus<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Queued(_) => f.write_str("Queued"),
            Self::Executing => f.write_str("Executing"),
            Self::Completed(_) => f.write_str("Completed"),
        }
    }
}
