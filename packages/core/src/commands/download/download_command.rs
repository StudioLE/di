use super::tag::Tag;
use crate::prelude::*;
use lofty::picture::Picture;
use tokio::task::spawn_blocking;

const CONCURRENCY: usize = 8;
const IMAGE_SIZE: u32 = 720;

pub struct DownloadCommand {
    paths: PathProvider,
    http: HttpClient,
    metadata: MetadataRepository,
}

impl DownloadCommand {
    #[must_use]
    pub fn new(paths: PathProvider, http: HttpClient, metadata: MetadataRepository) -> Self {
        Self {
            paths,
            http,
            metadata,
        }
    }

    pub async fn execute(&self, options: DownloadOptions) -> Result<(), Report<DownloadError>> {
        let feed = self
            .metadata
            .get_feed_by_slug(options.podcast_slug, Some(options.filter))
            .await
            .change_context(DownloadError::Repository)?
            .ok_or(DownloadError::NoPodcast)?;
        let results = self.process_episodes(feed).await;
        let mut episodes = Vec::new();
        let mut errors = Vec::new();
        for result in results {
            match result {
                Ok(episode) => episodes.push(episode),
                Err(e) => errors.push(e),
            }
        }
        info!("Downloaded audio files for {} episodes", episodes.len());
        if !errors.is_empty() {
            warn!("Skipped {} episodes due to failures", errors.len());
        }
        Ok(())
    }

    #[allow(clippy::as_conversions)]
    async fn process_episodes(
        &self,
        feed: PodcastFeed,
    ) -> Vec<Result<EpisodeInfo, Report<ProcessError>>> {
        let episodes: Vec<_> = feed
            .episodes
            .into_iter()
            .filter(|episode| {
                let exists = self
                    .paths
                    .get_audio_path(&feed.podcast.slug, episode)
                    .exists();
                if exists {
                    trace!(%episode, "Skipping existing");
                }
                !exists
            })
            .collect();
        let podcast = feed.podcast;
        debug!("Downloading audio files for {} episodes", episodes.len());
        stream::iter(episodes.into_iter().map(|episode| {
            let this = self;
            let podcast = podcast.clone();
            async move {
                let result = this
                    .process_episode(&podcast, episode.clone())
                    .await
                    .attach_episode(&episode);
                if let Err(e) = &result {
                    warn!("{e}");
                }
                result
            }
        }))
        .buffer_unordered(CONCURRENCY)
        .collect::<Vec<_>>()
        .await
    }

    async fn process_episode(
        &self,
        podcast: &PodcastInfo,
        episode: EpisodeInfo,
    ) -> Result<EpisodeInfo, Report<ProcessError>> {
        let path = self.download_episode(&episode).await?;
        let audio_path = self.copy_episode(&podcast.slug, &episode, &path).await?;
        let cover = self.download_image(&episode).await?;
        trace!(%episode, "Setting tags");
        Tag::execute(podcast, &episode, cover, &audio_path)
            .change_context(ProcessError::Tag)
            .attach_path(audio_path)?;
        Ok(episode)
    }

    async fn download_episode(
        &self,
        episode: &EpisodeInfo,
    ) -> Result<PathBuf, Report<ProcessError>> {
        self.http
            .get(&episode.get_source_url(), Some(MP3_EXTENSION))
            .await
            .change_context(ProcessError::DownloadAudio)
    }

    async fn copy_episode(
        &self,
        podcast_slug: &Slug,
        episode: &EpisodeInfo,
        source_path: &PathBuf,
    ) -> Result<PathBuf, Report<ProcessError>> {
        let destination_path = self.paths.get_audio_path(podcast_slug, episode);
        create_parent_dir_if_not_exist(&destination_path)
            .await
            .change_context(ProcessError::CreateDirectory)?;
        trace!(
            %episode,
            source = %source_path.display(),
            target = %destination_path.display(),
            "Copying audio"
        );
        copy(&source_path, &destination_path)
            .await
            .change_context(ProcessError::CopyAudio)
            .attach_with(|| {
                format!(
                    "Source: {}\nDestination: {}",
                    source_path.display(),
                    destination_path.display()
                )
            })?;
        Ok(destination_path)
    }

    async fn download_image(
        &self,
        episode: &EpisodeInfo,
    ) -> Result<Option<Picture>, Report<ProcessError>> {
        let Some(url) = &episode.get_image_url() else {
            return Ok(None);
        };
        trace!(%episode, "Downloading image");
        let extension = url.get_extension();
        let path = self
            .http
            .get(url, extension.as_deref())
            .await
            .change_context(ProcessError::DownloadImage)?;
        trace!(%episode, "Resizing image");
        let picture = spawn_blocking(move || -> Result<Picture, Report<ResizeError>> {
            Resize::new(&path)
                .attach_path(path)?
                .to_picture(IMAGE_SIZE, IMAGE_SIZE)
        })
        .await
        .change_context(ProcessError::Task)?
        .change_context(ProcessError::ResizeImage)?;
        trace!(%episode, "Resized image");
        Ok(Some(picture))
    }
}

#[derive(Clone, Debug, Error)]
pub enum DownloadError {
    #[error("Unable to get podcast")]
    Repository,
    #[error("Podcast does not exist")]
    NoPodcast,
}

#[derive(Clone, Debug, Error)]
pub enum ProcessError {
    #[error("Unable to download audio")]
    DownloadAudio,
    #[error("Unable to create directory")]
    CreateDirectory,
    #[error("Unable to copy audio file")]
    CopyAudio,
    #[error("Unable to download image")]
    DownloadImage,
    #[error("Failed to execute resize task")]
    Task,
    #[error("Unable to resize image")]
    ResizeImage,
    #[error("Unable to tag audio file")]
    Tag,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[traced_test]
    pub async fn download_command() {
        // Arrange
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");
        let command = DownloadCommand::new(services.paths, services.http, services.metadata);
        let options = DownloadOptions {
            podcast_slug: Slug::from_str("irl").expect("should be valid slug"),
            filter: FilterOptions {
                from_year: Some(2019),
                to_year: Some(2019),
                ..FilterOptions::default()
            },
        };

        // Act
        let result = command.execute(options).await;

        // Assert
        result.assert_ok_debug();
    }

    #[tokio::test]
    #[traced_test]
    pub async fn process_episode() {
        // Arrange
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");
        let feed = services
            .metadata
            .get_feed_by_slug(Slug::from_str("irl").expect("should be valid slug"), None)
            .await
            .expect("repository query should not fail")
            .expect("podcast should exist");
        let command = DownloadCommand::new(services.paths, services.http, services.metadata);
        let episode = feed
            .episodes
            .get(1)
            .expect("should be at least one episode")
            .clone();

        // Act
        let result = command.process_episode(&feed.podcast, episode).await;

        // Assert
        result.assert_ok_debug();
    }
}
