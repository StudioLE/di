use crate::prelude::*;

const CONCURRENCY: usize = 8;

define_commands!(Download(DownloadRequest));

pub struct DownloadCliCommand {
    metadata: Arc<MetadataRepository>,
    runner: Arc<CommandRunner<CommandInfo>>,
    progress: Arc<CliProgress<CommandInfo>>,
}

impl Service for DownloadCliCommand {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self::new(
            services.get_service().await?,
            services.get_service().await?,
            services.get_service().await?,
        ))
    }
}

impl DownloadCliCommand {
    #[must_use]
    pub fn new(
        metadata: Arc<MetadataRepository>,
        runner: Arc<CommandRunner<CommandInfo>>,
        progress: Arc<CliProgress<CommandInfo>>,
    ) -> Self {
        Self {
            metadata,
            runner,
            progress,
        }
    }

    #[allow(unreachable_patterns, clippy::match_wildcard_for_single_variants)]
    pub async fn execute(&self, options: DownloadOptions) -> Result<(), Report<DownloadCliError>> {
        let feed = self
            .metadata
            .get_feed_by_slug(options.podcast_slug, Some(options.filter))
            .await
            .change_context(DownloadCliError::Repository)?
            .ok_or(DownloadCliError::NoPodcast)?;
        let podcast = feed.podcast.primary_key;
        for episode in feed.episodes.iter() {
            let request = DownloadRequest::new(podcast, episode.primary_key);
            self.runner
                .queue_request(request)
                .await
                .expect("should be able to queue request");
        }
        self.progress.start().await;
        self.runner.start(CONCURRENCY).await;
        self.runner.drain().await;
        let results = self.runner.drain_results().await;
        self.progress.finish().await;
        let mut episodes = Vec::new();
        let mut errors = Vec::new();
        for result in results {
            match result {
                CommandResult::Download(_, Ok(episode)) => episodes.push(episode),
                CommandResult::Download(_, Err(e)) => errors.push(e),
                _ => unreachable!("Should only get download results"),
            }
        }
        info!("Downloaded audio files for {} episodes", episodes.len());
        if !errors.is_empty() {
            warn!("Skipped {} episodes due to failures", errors.len());
            for error in errors {
                warn!("{error:?}");
            }
        }
        Ok(())
    }
}

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
    #[traced_test]
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

        // Act
        let result = command.execute(options).await;

        // Assert
        result.assert_ok_debug();
    }
}
