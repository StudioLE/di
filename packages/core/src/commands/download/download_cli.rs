use crate::prelude::*;

/// Maximum concurrent downloads.
const CONCURRENCY: usize = 8;

/// CLI command for batch downloading episodes.
///
/// Queues multiple [`DownloadRequest`]s based on filter criteria and
/// executes them concurrently with a progress bar.
#[derive(Service)]
pub struct DownloadCliCommand {
    metadata: Arc<MetadataRepository>,
    runner: Arc<CommandRunner<CommandInfo>>,
    progress: Arc<CliProgress<CommandInfo>>,
}

impl DownloadCliCommand {
    /// Download episodes matching the filter criteria.
    #[allow(unreachable_patterns, clippy::match_wildcard_for_single_variants)]
    pub async fn execute(&self, options: DownloadOptions) -> Result<(), Report<DownloadCliError>> {
        let feed = self
            .metadata
            .get_feed_by_slug(options.podcast_slug, Some(options.filter))
            .await
            .change_context(DownloadCliError::Repository)?
            .ok_or(DownloadCliError::NoPodcast)?;
        let podcast = feed.podcast.primary_key;
        self.progress.start().await;
        for episode in feed.episodes.iter() {
            let request = DownloadRequest::new(podcast, episode.primary_key);
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
                CommandStatus::Succeeded(CommandSuccess::Download(_response)) => {
                    requests.push(request);
                }
                CommandStatus::Failed(CommandFailure::Download(e)) => errors.push(e),
                _ => unreachable!("Should only get download results"),
            }
        }
        info!("Downloaded audio files for {} episodes", requests.len());
        if !errors.is_empty() {
            warn!("Skipped {} episodes due to failures", errors.len());
            for error in errors {
                warn!("{error:?}");
            }
        }
        Ok(())
    }
}

/// Errors from [`DownloadCliCommand`].
#[derive(Clone, Debug, Error)]
pub enum DownloadCliError {
    #[error("Unable to get podcast")]
    Repository,
    #[error("Podcast does not exist")]
    NoPodcast,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::as_conversions, clippy::cast_possible_wrap)]
    use super::*;

    #[tokio::test]
    #[serial]
    pub async fn download_command() {
        // Arrange
        let services = TestServiceProvider::create().await;
        let command = services
            .get_service::<DownloadCliCommand>()
            .await
            .expect("should be able to get command");
        let options = DownloadOptions {
            podcast_slug: MetadataRepositoryExample::podcast_slug(),
            filter: FilterOptions {
                year: Some(MetadataRepositoryExample::START_YEAR as i32),
                season: Some(1),
                ..FilterOptions::default()
            },
        };
        let _logger = init_test_logger();

        // Act
        let result = command.execute(options).await;

        // Assert
        result.assert_ok_debug();
    }
}
