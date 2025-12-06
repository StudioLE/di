#![allow(dead_code)]

use crate::prelude::*;
use tokio::sync::futures::Notified;

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
    mediator: Arc<WorkerMediator<T>>,
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
        mediator: Arc<WorkerMediator<T>>,
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

    /// Stop workers after draining the queue
    pub async fn drain(&self) {
        self.mediator.set_status(RunnerStatus::Draining).await;
        self.workers.wait_for_stop().await;
    }

    /// Stop workers after their current work is complete
    pub async fn stop(&self) {
        self.mediator.set_status(RunnerStatus::Stopping).await;
        self.workers.wait_for_stop().await;
    }

    /// Add a request to the command queue and notify a worker.
    pub async fn queue_request<R: Executable + Send + Sync + 'static>(
        &self,
        request: R,
    ) -> Result<(), Report<QueueError>> {
        let command = self.registry.resolve(request)?;
        self.queue_command(command).await;
        Ok(())
    }

    /// Add a command to the queue and notify a worker.
    async fn queue_command(&self, command: T::Command) {
        let mut queue = self.mediator.queue.lock().await;
        queue.push_back(command);
        drop(queue);
        self.mediator.notify_workers.notify_one();
        let mut queued = self.mediator.all_time_queued.lock().await;
        *queued += 1;
        drop(queued);
        self.mediator.notify_progress.notify_one();
    }

    /// Get the number of items in the queue
    pub async fn get_queue_length(&self) -> usize {
        let queue = self.mediator.queue.lock().await;
        queue.len()
    }

    /// Get the number of items in the queue
    pub async fn get_results_length(&self) -> usize {
        let queue = self.mediator.results.lock().await;
        queue.len()
    }

    /// Drain all the results
    pub async fn drain_results(&self) -> Vec<T::Result> {
        let mut results = self.mediator.results.lock().await;
        results.drain(..).collect()
    }

    /// Wait for progress to be reported
    pub fn wait_for_progress(&self) -> Notified<'_> {
        self.mediator.notify_progress.notified()
    }

    /// Number of commands queued over time
    pub async fn get_all_time_queued(&self) -> usize {
        let queued_guard = self.mediator.all_time_queued.lock().await;
        *queued_guard
    }

    /// Number of commands completed over time
    pub async fn get_all_time_completed(&self) -> usize {
        let completed_guard = self.mediator.all_time_completed.lock().await;
        *completed_guard
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
        init_elapsed_logger();

        // Arrange
        let services = ServiceProvider::new()
            .with_commands()
            .await
            .expect("should be able to create services with commands");
        // let mut registry: CommandRegistry<CommandInfo> = CommandRegistry::new();
        // let handler = services.get_service::<DelayHandler>().await.expect("should be able to get handler");
        // registry.register::<DelayRequest>(handler);
        // services.add_instance(registry);
        let runner = services
            .get_service::<CommandRunner<CommandInfo>>()
            .await
            .expect("should be able to get runner");

        // Act
        runner.workers.start(WORKER_COUNT).await;

        info!("Adding {A_COUNT} commands to queue");
        for i in 1..=A_COUNT {
            let request = DelayRequest::new(format!("A{i}"), A_DURATON);
            let command = runner
                .registry
                .resolve(request)
                .expect("should be able to add command");
            runner.queue_command(command).await;
        }
        info!("Added {A_COUNT} commands to queue");

        // Assert
        let length = runner.get_queue_length().await;
        debug!("Queue: {length}");
        assert_eq!(length, A_COUNT, "Queue immediately after sending batch A");

        wait(50).await;
        let length = runner.get_queue_length().await;
        debug!("Queue: {length}");
        assert_ne!(length, 0, "Queue soon after adding batch A");

        wait(A_TOTAL_DURATON + 100).await;
        let length = runner.get_queue_length().await;
        debug!("Queue: {length}");
        assert_eq!(length, 0, "Queue after batch A should have completed");

        info!("Adding {B_COUNT} commands to queue");
        for i in 1..=B_COUNT {
            let request = DelayRequest::new(format!("B{i}"), B_DURATON);
            let command = runner
                .registry
                .resolve(request)
                .expect("should be able to add command");
            runner.queue_command(command).await;
        }
        info!("Added {B_COUNT} commands to queue");

        wait(50).await;
        info!("Requesting stop");
        runner.workers.stop().await;
        info!("Completed stop");

        let length = runner.get_queue_length().await;
        debug!("Queue: {length}");
        assert_eq!(length, 6, "Queue after stop");
        let length = runner.get_results_length().await;
        debug!("Results: {length}");
        assert_eq!(length, 14, "Results after stop");
    }

    async fn wait(wait: u64) {
        sleep(Duration::from_millis(wait)).await;
        info!("Waiting {wait} ms");
    }
}
