use crate::prelude::*;

/// Maximum concurrent fetches.
const CONCURRENCY: usize = 8;

/// CLI command for fetching existing podcasts.
///
/// Queues multiple [`FetchRequest`] and executes them concurrently
/// with a progress bar.
#[derive(Service)]
pub struct FetchCliCommand {
    metadata: Arc<MetadataRepository>,
    runner: Arc<CommandRunner<CommandInfo>>,
    progress: Arc<CliProgress<CommandInfo>>,
}

impl FetchCliCommand {
    /// Fetch podcasts matching the options.
    ///
    /// - If a podcast slug is provided, fetches that single podcast.
    /// - If no slug is provided, fetches all podcasts.
    pub async fn execute(&self, options: FetchOptions) -> Result<(), Report<FetchCliError>> {
        let slugs = match options.podcast {
            Some(slug) => vec![slug],
            None => self
                .metadata
                .get_all_podcast_slugs()
                .await
                .change_context(FetchCliError::Repository)?,
        };
        self.fetch(slugs).await
    }

    /// Fetch podcasts by their slugs.
    #[allow(unreachable_patterns, clippy::match_wildcard_for_single_variants)]
    async fn fetch(&self, slugs: Vec<Slug>) -> Result<(), Report<FetchCliError>> {
        if slugs.is_empty() {
            info!("No podcasts found");
            return Ok(());
        }
        trace!(count = slugs.len(), "Fetching podcasts");
        self.progress.start().await;
        for slug in slugs {
            let request = FetchRequest { slug };
            self.runner
                .queue_request(request)
                .await
                .expect("should be able to queue request");
        }
        self.runner.start(CONCURRENCY).await;
        self.runner.drain().await;
        self.progress.finish().await;
        let results = self.runner.get_commands().await;
        let mut requests = Vec::new();
        let mut errors = Vec::new();
        for (request, status) in results.iter() {
            match status {
                CommandStatus::Succeeded(CommandSuccess::Fetch(_response)) => {
                    requests.push(request);
                }
                CommandStatus::Failed(CommandFailure::Fetch(e)) => errors.push(e),
                _ => unreachable!("Should only get fetch results"),
            }
        }
        info!("Fetched {} podcasts", requests.len());
        if !errors.is_empty() {
            warn!("Failed to fetch {} podcasts", errors.len());
            for error in errors {
                warn!("{error:?}");
            }
        }
        Ok(())
    }
}

/// Errors from [`FetchCliCommand`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum FetchCliError {
    /// Unable to query the database.
    #[error("Unable to query database")]
    Repository,
}
