use crate::prelude::*;
use indicatif::ProgressBar;
use tokio::spawn;

pub struct CliProgress<T: ICommandInfo> {
    runner: Arc<CommandRunner<T>>,
    progress: Arc<ProgressBar>,
    handle: Mutex<Option<JoinHandle<()>>>,
    finished: Arc<Mutex<bool>>,
}

impl<T: ICommandInfo + 'static> CliProgress<T> {
    #[must_use]
    pub fn new(runner: Arc<CommandRunner<T>>) -> Self {
        Self {
            runner,
            progress: Arc::new(ProgressBar::new(0)),
            handle: Mutex::default(),
            finished: Arc::new(Mutex::new(false)),
        }
    }

    #[allow(clippy::as_conversions)]
    pub async fn start(&self) {
        let mut handle_guard = self.handle.lock().await;
        if handle_guard.is_some() {
            return;
        }
        let runner = self.runner.clone();
        let progress = self.progress.clone();
        let finished = self.finished.clone();
        let handle = spawn(async move {
            loop {
                let queued = runner.get_all_time_queued().await;
                let completed = runner.get_all_time_completed().await;
                progress.set_length(queued as u64);
                progress.set_position(completed as u64);
                runner.wait_for_progress().await;
                if *finished.lock().await {
                    break;
                }
            }
        });
        *handle_guard = Some(handle);
    }

    pub async fn finish(&self) {
        let mut finished_guard = self.finished.lock().await;
        *finished_guard = true;
        drop(finished_guard);
        let mut handle_guard = self.handle.lock().await;
        if let Some(handle) = handle_guard.take() {
            handle.abort();
        }
        drop(handle_guard);
        self.progress.finish();
    }
}

impl<T: ICommandInfo + 'static> Service for CliProgress<T> {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self::new(services.get_service().await?))
    }
}
