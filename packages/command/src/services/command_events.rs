use crate::prelude::*;
use tokio::spawn;
use tokio::sync::broadcast::error::RecvError;
use tracing::{error, warn};

pub struct CommandEvents<T: ICommandInfo> {
    mediator: Arc<CommandMediator<T>>,
    events: Arc<Mutex<Vec<T::Event>>>,
    handle: Mutex<Option<JoinHandle<()>>>,
}

#[derive(Debug, Default)]
pub struct CommandEventCounts {
    pub queued: usize,
    pub executing: usize,
    pub succeeded: usize,
    pub failed: usize,
}

impl<T: ICommandInfo + 'static> CommandEvents<T> {
    #[must_use]
    pub fn new(mediator: Arc<CommandMediator<T>>) -> Self {
        Self {
            mediator,
            events: Arc::default(),
            handle: Mutex::default(),
        }
    }

    pub async fn start(&self) {
        let mut handle_guard = self.handle.lock().await;
        if handle_guard.is_some() {
            return;
        }
        let mediator = self.mediator.clone();
        let mut receiver = mediator.subscribe();
        let events = self.events.clone();
        let handle = spawn(async move {
            loop {
                match receiver.recv().await {
                    Err(RecvError::Lagged(count)) => {
                        warn!("CommandEvents missed {count} events due to lagging");
                    }
                    Err(RecvError::Closed) => {
                        error!("Event pipe was closed. CommandEvents can't proceed.");
                        break;
                    }
                    Ok(event) => {
                        let mut events_guard = events.lock().await;
                        events_guard.push(event);
                        drop(events_guard);
                    }
                }
            }
        });
        *handle_guard = Some(handle);
    }

    pub async fn get(&self) -> MutexGuard<'_, Vec<T::Event>> {
        self.events.lock().await
    }

    pub async fn count(&self) -> CommandEventCounts {
        let mut counts = CommandEventCounts::default();
        let events = self.events.lock().await;
        for event in events.iter() {
            match event.get_kind() {
                EventKind::Queued => counts.queued += 1,
                EventKind::Executing => counts.executing += 1,
                EventKind::Succeeded => counts.succeeded += 1,
                EventKind::Failed => counts.failed += 1,
            }
        }
        counts
    }
}

impl CommandEventCounts {
    /// Estimate the number of commands currently queued.
    ///
    /// For this to be accurate the [`CommandEvents`] must be started before any events occur.
    ///
    /// None is returned if the subtraction overflows.
    #[must_use]
    pub fn get_currently_queued(&self) -> Option<usize> {
        self.queued.checked_sub(self.executing)
    }

    /// Estimate the number of commands currently queued.
    ///
    /// For this to be accurate the [`CommandEvents`] must be started before any events occur.
    ///
    /// None is returned if the subtraction overflows.
    #[must_use]
    pub fn get_currently_executing(&self) -> Option<usize> {
        self.executing
            .checked_sub(self.succeeded)?
            .checked_sub(self.failed)
    }
}

impl<T: ICommandInfo + 'static> Service for CommandEvents<T> {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self::new(services.get_service().await?))
    }
}
