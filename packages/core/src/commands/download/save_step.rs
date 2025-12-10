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
        let query = self.update_episode_query(episode_key, file_path, image_path);
        let _ = self.db.execute_raw(query).await?;
        Ok(())
    }

    fn update_episode_query(
        &self,
        episode_key: EpisodeKey,
        file_path: PathBuf,
        image_path: Option<PathBuf>,
    ) -> Statement {
        let model = episode::ActiveModel {
            primary_key: Set(episode_key),
            file_sub_path: Set(Some(PathWrapper::from(file_path))),
            image_sub_path: Set(image_path.map(PathWrapper::from)),
            ..Default::default()
        };
        episode::Entity::update(model)
            .validate()
            .expect("query should be valid")
            .build(self.db.get_database_backend())
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[tokio::test]
    pub async fn update_episode_query() {
        // Arrange
        let metadata = MetadataRepositoryExample::create().await;
        let file_path = PathBuf::from("path/to/audio.mp3");
        let image_path = Some(PathBuf::from("path/to/image.jpg"));

        // Act
        let statement = metadata.update_episode_query(EPISODE_KEY, file_path, image_path);

        // Assert
        let sql = format_sql(&statement);
        assert_snapshot!(sql);
    }

    #[tokio::test]
    pub async fn update_episode() {
        // Arrange
        let metadata = MetadataRepositoryExample::create().await;
        let file_path = PathBuf::from("path/to/audio.mp3");
        let image_path = Some(PathBuf::from("path/to/image.jpg"));
        let _logger = init_test_logger();
        let slug = podcast_slug();

        // Act
        let result = metadata
            .update_episode(EPISODE_KEY, file_path.clone(), image_path.clone())
            .await;

        // Assert
        result.assert_ok_debug();
        let episode = metadata
            .get_episode(slug, EPISODE_KEY)
            .await
            .expect("should be able to get episode")
            .expect("episode should exist");
        assert_eq!(episode.file_sub_path, Some(PathWrapper::from(file_path)));
        assert_eq!(episode.image_sub_path, image_path.map(PathWrapper::from));
    }
}
