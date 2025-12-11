use crate::prelude::*;

/// A mediator between the [`CommandRunner`], [`Worker`] and [`CliProgress`] services.
pub struct CommandMediator<T: ICommandInfo> {
    /// Number of commands queued over time
    progress: Mutex<CommandProgress>,
    /// Queue of commands to execute
    queue: Mutex<VecDeque<T::Request>>,
    /// Current status of the runner
    commands: Mutex<HashMap<T::Request, CommandStatus<T>>>,
    /// Notify workers when new work is available
    notify_workers: Notify,
    /// Notify progress subscribers when work is queued, executing, or completed
    notify_progress: Notify,
    /// Current status of the runner
    runner_status: Mutex<RunnerStatus>,
}

impl<T: ICommandInfo + 'static> Service for CommandMediator<T> {
    type Error = Infallible;

    async fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self::new())
    }
}

impl<T: ICommandInfo> CommandMediator<T> {
    pub(super) fn new() -> Self {
        Self {
            progress: Mutex::default(),
            queue: Mutex::default(),
            notify_workers: Notify::default(),
            notify_progress: Notify::default(),
            runner_status: Mutex::default(),
            commands: Mutex::default(),
        }
    }

    async fn get_runner_status(&self) -> RunnerStatus {
        *self.runner_status.lock().await
    }

    async fn update_progress(&self, callback: fn(&mut MutexGuard<CommandProgress>)) {
        let mut progress = self.progress.lock().await;
        callback(&mut progress);
        drop(progress);
        self.notify_progress.notify_waiters();
    }
}

// Implementation for `CommandRunner`
impl<T: ICommandInfo> CommandMediator<T> {
    pub(super) async fn set_runner_status(&self, status: RunnerStatus) {
        let mut status_guard = self.runner_status.lock().await;
        *status_guard = status;
        drop(status_guard);
        self.notify_workers.notify_waiters();
    }

    /// Add a command to the queue.
    ///
    /// If the request is already queued or executing then it's ignored and `false` is returned.
    ///
    /// If added to the queue then progress is updated and subscribers are notified.
    pub(super) async fn queue(&self, request: T::Request, command: T::Command) -> bool {
        let mut commands = self.commands.lock().await;
        if let Some(CommandStatus::Queued(_) | CommandStatus::Executing) = commands.get(&request) {
            return false;
        }
        commands.insert(request.clone(), CommandStatus::Queued(command));
        drop(commands);
        self.update_progress(|progress| {
            progress.queued += 1;
            progress.total += 1;
        })
        .await;
        let mut queue = self.queue.lock().await;
        queue.push_back(request);
        drop(queue);
        self.notify_workers.notify_one();
        true
    }

    /// Get the commands.
    ///
    /// Note: The [`MutexGuard`] must be dropped or the [`Worker`] will be unable to finish
    /// execution.
    pub(super) async fn get_commands(
        &self,
    ) -> MutexGuard<'_, HashMap<T::Request, CommandStatus<T>>> {
        self.commands.lock().await
    }
}

// Implementation for `Worker`
impl<T: ICommandInfo> CommandMediator<T> {
    /// Get the next instruction.
    #[allow(clippy::panic)]
    pub(super) async fn get_instruction(&self) -> Instruction<'_, T> {
        let mut queue_guard = self.queue.lock().await;
        if self.get_runner_status().await == RunnerStatus::Stopping {
            return Instruction::Stop;
        }
        if let Some(request) = queue_guard.pop_front() {
            drop(queue_guard);
            self.update_progress(|progress| {
                progress.queued -= 1;
                progress.executing += 1;
            })
            .await;
            let mut commands = self.commands.lock().await;
            let option = commands.insert(request.clone(), CommandStatus::Executing);
            drop(commands);
            let Some(CommandStatus::Queued(command)) = option else {
                panic!("command should be queued but was {option:?}");
            };
            return Instruction::Execute(request, command);
        }
        drop(queue_guard);
        if self.get_runner_status().await == RunnerStatus::Draining {
            return Instruction::Stop;
        }
        Instruction::Wait(self.notify_workers.notified())
    }

    /// Add the result of a completed execution.
    pub(super) async fn completed(&self, request: T::Request, result: T::Result) {
        let mut commands = self.commands.lock().await;
        commands.insert(request, CommandStatus::Completed(result));
        drop(commands);
        self.update_progress(|progress| {
            progress.executing -= 1;
            progress.completed += 1;
        })
        .await;
    }
}

// Implementation for `Progress` subscribers
impl<T: ICommandInfo> CommandMediator<T> {
    /// Get the current progress.
    pub async fn get_progress(&self) -> CommandProgress {
        let guard = self.progress.lock().await;
        (*guard).clone()
    }

    /// Wait for progress to be reported
    pub async fn wait_for_progress(&self) -> CommandProgress {
        self.notify_progress.notified().await;
        self.get_progress().await
    }
}
