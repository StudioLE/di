use crate::prelude::*;
use sea_orm::Selector;
use sea_orm::*;

impl DownloadHandler {
    pub(super) async fn context_step(
        &self,
        request: &DownloadRequest,
    ) -> Result<DownloadContext, Report<DownloadError>> {
        let (podcast, episodes) = self
            .metadata
            .get_download_podcast(request.podcast, request.episode)
            .await?;
        let podcasts_dir = self.paths.get_podcasts_dir();
        Ok(DownloadContext::new(podcast, episodes, podcasts_dir))
    }
}

impl MetadataRepository {
    /// Get a podcast with minimal info for the podcast page.
    async fn get_download_podcast(
        &self,
        podcast_key: PodcastKey,
        episode_key: EpisodeKey,
    ) -> Result<(DownloadPodcastPartial, DownloadEpisodePartial), Report<DownloadError>> {
        let podcast = get_download_podcast_query(podcast_key)
            .one(&self.db)
            .await
            .change_context(DownloadError::GetPodcast)?
            .ok_or(DownloadError::NoPodcast)?;
        let episode = get_download_episode_query(podcast_key, episode_key)
            .one(&self.db)
            .await
            .change_context(DownloadError::GetEpisode)?
            .ok_or(DownloadError::NoEpisode)?;
        Ok((podcast, episode))
    }
}

fn get_download_podcast_query(
    podcast_key: PodcastKey,
) -> Selector<SelectModel<DownloadPodcastPartial>> {
    podcast::Entity::find_by_id(podcast_key)
        .select_only()
        .columns([
            podcast::Column::PrimaryKey,
            podcast::Column::Slug,
            podcast::Column::Title,
        ])
        .into_model()
}

fn get_download_episode_query(
    podcast_key: PodcastKey,
    episode_key: EpisodeKey,
) -> Selector<SelectModel<DownloadEpisodePartial>> {
    episode::Entity::find_by_id(episode_key)
        .has_related(podcast::Entity, podcast::Column::PrimaryKey.eq(podcast_key))
        .select_only()
        .columns([
            episode::Column::PrimaryKey,
            episode::Column::Title,
            episode::Column::FileSubPath,
            episode::Column::ImageSubPath,
            episode::Column::SourceUrl,
            episode::Column::SourceContentType,
            episode::Column::PublishedAt,
            episode::Column::Image,
            episode::Column::Episode,
            episode::Column::Season,
        ])
        .into_model()
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    pub fn _get_download_podcast_query() {
        // Arrange
        // Act
        let statement = get_download_podcast_query(PODCAST_KEY).into_statement(DB_BACKEND);

        // Assert
        let sql = format_sql(&statement);
        assert_snapshot!(sql);
    }

    #[test]
    pub fn _get_download_episode_query() {
        // Arrange
        // Act
        let statement =
            get_download_episode_query(PODCAST_KEY, EPISODE_KEY).into_statement(DB_BACKEND);

        // Assert
        let sql = format_sql(&statement);
        assert_snapshot!(sql);
    }

    #[tokio::test]
    #[traced_test]
    pub async fn get_download_podcast() {
        // Arrange
        let metadata = MetadataRepositoryExample::create().await;

        // Act
        let result = metadata
            .get_download_podcast(PODCAST_KEY, EPISODE_KEY)
            .await;

        // Assert
        let (podcast, episodes) = result.assert_ok_debug();
        assert_yaml_snapshot!((podcast, episodes));
    }
}
