use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum EventKind {
    Queued,
    Executing,
    Succeeded,
    Failed,
}
