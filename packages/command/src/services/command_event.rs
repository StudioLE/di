use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct CommandEvent<Req: IRequest> {
    pub request: Req,
    pub kind: EventKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventKind {
    Queued,
    Executing,
    Succeeded,
    Failed,
}

impl<Req: IRequest> CommandEvent<Req> {
    pub fn new(request: Req, kind: EventKind) -> Self {
        Self { request, kind }
    }
}
