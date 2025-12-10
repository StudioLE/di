use crate::prelude::*;
use sea_orm::*;

impl DownloadHandler {
    pub(super) async fn save_step(
        &self,
        context: &DownloadContext,
    ) -> Result<(), Report<DownloadError>> {
        let podcasts_dir = self.paths.get_podcasts_dir();
        let file_path = context
            .file_path
            .clone()
            .strip_prefix(&podcasts_dir)
            .expect("path should have prefix")
            .to_path_buf();
        let image_path = context.image_path.clone().map(|path| {
            path.strip_prefix(&podcasts_dir)
                .expect("path should have prefix")
                .to_path_buf()
        });
        self.metadata
            .update_episode(context.episode.primary_key, file_path, image_path)
            .await
            .change_context(DownloadError::Save)
    }
}

impl MetadataRepository {
    /// Set the file path and image path for an episode.
    async fn update_episode(
        &self,
        episode_key: EpisodeKey,
        file_path: PathBuf,
        image_path: Option<PathBuf>,
    ) -> Result<(), DbErr> {
        let _model = update_episode_query(episode_key, file_path, image_path)
            .exec(&self.db)
            .await;
        Ok(())
    }
}

fn update_episode_query(
    episode_key: EpisodeKey,
    file_path: PathBuf,
    image_path: Option<PathBuf>,
) -> UpdateOne<episode::ActiveModel> {
    let model = episode::ActiveModel {
        primary_key: Set(episode_key),
        file_sub_path: Set(Some(PathWrapper::from(file_path))),
        image_sub_path: Set(image_path.map(PathWrapper::from)),
        ..Default::default()
    };
    episode::Entity::update(model)
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    pub fn _update_episode_query() {
        // Arrange
        let file_path = PathBuf::from("path/to/audio.mp3");
        let image_path = Some(PathBuf::from("path/to/image.jpg"));

        // Act
        let statement = update_episode_query(EPISODE_KEY, file_path, image_path)
            .validate()
            .expect("should be valid")
            .build(DB_BACKEND);

        // Assert
        let sql = format_sql(&statement);
        assert_snapshot!(sql);
    }

    #[tokio::test]
    pub async fn update_episode() {
        // Arrange
        let _logger = init_test_logger();
        let metadata = MetadataRepositoryExample::create().await;
        let slug = MetadataRepositoryExample::podcast_slug();

        // Act
        let result = metadata.get_podcast(slug).await;

        // Assert
        let (podcast, episodes) = result.assert_ok_debug().expect("Podcast should exist");
        assert_yaml_snapshot!((podcast, episodes));
    }
}
