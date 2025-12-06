use crate::prelude::*;
use tokio::spawn;
use tokio::sync::futures::Notified;

pub type WorkerId = usize;

/// An instruction sent to a [`Worker`].
pub enum Instruction<'a, T: ICommandInfo> {
    Wait(Notified<'a>),
    Stop,
    Execute(T::Command),
}

/// A worker that executes commands
///
/// The worker is instructed by a [`WorkerMediator`].
pub struct Worker {
    id: WorkerId,
    handle: JoinHandle<()>,
}

impl Worker {
    pub(super) fn new<T: ICommandInfo + 'static>(
        id: WorkerId,
        mediator: Arc<WorkerMediator<T>>,
    ) -> Self {
        let handle = spawn(async move {
            internal_loop(mediator, id).await;
        });
        Self { id, handle }
    }

    /// Wait for the worker to stop
    pub(super) async fn wait_for_stop(self) {
        let _ = self.handle.await;
    }
}

async fn internal_loop<T: ICommandInfo>(mediator: Arc<WorkerMediator<T>>, worker: WorkerId) {
    loop {
        match mediator.get_instruction().await {
            Instruction::Execute(command) => {
                let command_id = command.to_string();
                trace!(worker, %command, "Executing");
                let response = command.execute().await;
                mediator.add_result(response).await;
                trace!(worker, command = command_id, "Executed");
            }
            Instruction::Wait(notified) => {
                trace!(worker, "Waiting");
                notified.await;
            }
            Instruction::Stop => {
                trace!(worker, "Stopping");
                break;
            }
        }
    }
    trace!(worker, "Stopped");
}

impl Display for Worker {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "Worker {:02}", self.id)
    }
}
