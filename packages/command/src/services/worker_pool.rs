use crate::prelude::*;

pub struct WorkerPool<T: ICommandInfo> {
    mediator: Arc<CommandMediator<T>>,
    /// Current worker index
    ///
    /// Used to ensure each worker has a unique ID even if additional workers are started
    latest_worker_index: Arc<Mutex<WorkerId>>,
    /// Workers
    workers: Arc<Mutex<Vec<Worker>>>,
}

impl<T: ICommandInfo + 'static> Service for WorkerPool<T> {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self::new(services.get_service().await?))
    }
}

impl<T: ICommandInfo + 'static> WorkerPool<T> {
    #[must_use]
    pub fn new(mediator: Arc<CommandMediator<T>>) -> Self {
        Self {
            mediator,
            latest_worker_index: Arc::default(),
            workers: Arc::default(),
        }
    }

    /// Start any number of workers.
    ///
    /// Each worker will have a unique ID.
    ///
    /// Status will be set to `Running`.
    pub(super) async fn start(&self, worker_count: usize) {
        let mut index_guard = self.latest_worker_index.lock().await;
        let start = *index_guard + 1;
        let end = start + worker_count;
        *index_guard = end;
        drop(index_guard);
        self.mediator.set_runner_status(RunnerStatus::Running).await;
        let mut handles = Vec::with_capacity(worker_count);
        for worker_id in start..=end {
            let handle = Worker::new(worker_id, self.mediator.clone());
            handles.push(handle);
        }
        let mut workers_guard = self.workers.lock().await;
        workers_guard.append(&mut handles);
        drop(workers_guard);
    }

    /// Stop workers after draining the queue
    pub async fn drain(&self) {
        self.mediator
            .set_runner_status(RunnerStatus::Draining)
            .await;
        self.wait_for_stop().await;
    }

    /// Stop workers after their current work is complete
    pub async fn stop(&self) {
        self.mediator
            .set_runner_status(RunnerStatus::Stopping)
            .await;
        self.wait_for_stop().await;
    }

    pub(super) async fn wait_for_stop(&self) {
        let mut workers_guard = self.workers.lock().await;
        let workers = take(&mut *workers_guard);
        drop(workers_guard);
        for worker in workers {
            worker.wait_for_stop().await;
        }
    }
}
