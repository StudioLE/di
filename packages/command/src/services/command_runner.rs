#![allow(dead_code)]

use crate::prelude::*;
use tokio::sync::MutexGuard;

#[derive(Clone, Copy, Debug, Default, Eq, Error, PartialEq)]
pub enum RunnerStatus {
    #[default]
    #[error("Runner is stopped")]
    Stopped,
    #[error("Stopping when the active commands are complete")]
    Stopping,
    #[error("Stopping when the queue is empty")]
    Draining,
    #[error("Running")]
    Running,
}

pub struct CommandRunner<T: ICommandInfo> {
    mediator: Arc<CommandMediator<T>>,
    registry: Arc<CommandRegistry<T>>,
    workers: Arc<WorkerPool<T>>,
}

impl<T: ICommandInfo + 'static> Service for CommandRunner<T> {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self::new(
            services.get_service().await?,
            services.get_service().await?,
            services.get_service().await?,
        ))
    }
}

impl<T: ICommandInfo + 'static> CommandRunner<T> {
    #[must_use]
    pub fn new(
        mediator: Arc<CommandMediator<T>>,
        registry: Arc<CommandRegistry<T>>,
        workers: Arc<WorkerPool<T>>,
    ) -> Self {
        Self {
            mediator,
            registry,
            workers,
        }
    }

    /// Start any number of workers.
    ///
    /// Each worker will have a unique ID.
    ///
    /// Status will be set to `Running`.
    pub async fn start(&self, worker_count: usize) {
        self.workers.start(worker_count).await;
    }

    /// Stop workers after draining the queue.
    pub async fn drain(&self) {
        self.mediator
            .set_runner_status(RunnerStatus::Draining)
            .await;
        self.workers.wait_for_stop().await;
    }

    /// Stop workers after their current work is complete
    pub async fn stop(&self) {
        self.mediator
            .set_runner_status(RunnerStatus::Stopping)
            .await;
        self.workers.wait_for_stop().await;
    }

    /// Queue a command as a request.
    pub async fn queue_request<R: Executable + Into<T::Request> + Send + Sync + 'static>(
        &self,
        request: R,
    ) -> Result<(), Report<QueueError>> {
        let command = self.registry.resolve(request.clone())?;
        self.mediator.queue(request.into(), command).await;
        Ok(())
    }

    /// Get the commands.
    ///
    /// Note: The [`MutexGuard`] must be dropped or the [`Worker`] will be unable to finish
    /// execution.
    pub async fn get_commands(&self) -> MutexGuard<'_, HashMap<T::Request, CommandStatus<T>>> {
        self.mediator.get_commands().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;
    use tokio::time::sleep;

    const WORKER_COUNT: usize = 3;
    const A_COUNT: usize = 10;
    const B_COUNT: usize = 10;
    const A_DURATON: u64 = 100;
    const B_DURATON: u64 = 100;
    #[allow(clippy::as_conversions, clippy::integer_division)]
    const A_TOTAL_DURATON: u64 = (A_COUNT / WORKER_COUNT) as u64 * A_DURATON;

    #[tokio::test]
    async fn command_runner() {
        // Arrange
        let services = ServiceProvider::new()
            .with_commands()
            .await
            .expect("should be able to create services with commands");
        let runner = services
            .get_service::<CommandRunner<CommandInfo>>()
            .await
            .expect("should be able to get runner");
        let mediator = services
            .get_service::<CommandMediator<CommandInfo>>()
            .await
            .expect("should be able to get mediator");
        let _logger = init_test_logger();

        // Act
        runner.workers.start(WORKER_COUNT).await;

        info!("Adding {A_COUNT} commands to queue");
        for i in 1..=A_COUNT {
            let request = DelayRequest::new(format!("A{i}"), A_DURATON);
            runner
                .queue_request(request)
                .await
                .expect("should be able to queue command");
        }
        info!("Added {A_COUNT} commands to queue");

        // Assert
        let length = mediator.get_progress().await.queued;
        debug!("Queue: {length}");
        assert_eq!(length, A_COUNT, "Queue immediately after sending batch A");

        wait(50).await;
        let length = mediator.get_progress().await.queued;
        debug!("Queue: {length}");
        assert_ne!(length, 0, "Queue soon after adding batch A");

        wait(A_TOTAL_DURATON + 100).await;
        let length = mediator.get_progress().await.queued;
        debug!("Queue: {length}");
        assert_eq!(length, 0, "Queue after batch A should have completed");

        info!("Adding {B_COUNT} commands to queue");
        for i in 1..=B_COUNT {
            let request = DelayRequest::new(format!("B{i}"), B_DURATON);
            runner
                .queue_request(request)
                .await
                .expect("should be able to queue command");
        }
        info!("Added {B_COUNT} commands to queue");

        wait(50).await;
        info!("Requesting stop");
        runner.workers.stop().await;
        info!("Completed stop");

        let progress = mediator.get_progress().await;
        debug!("Queue: {}", progress.queued);
        assert_eq!(progress.queued, 6, "Queue after stop");
        debug!("Completed: {}", progress.completed);
        assert_eq!(progress.completed, 14, "Completed after stop");
    }

    async fn wait(wait: u64) {
        sleep(Duration::from_millis(wait)).await;
        info!("Waiting {wait} ms");
    }
}
