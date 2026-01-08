use crate::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::*;

impl MetadataRepository {
    /// Create a new podcast feed.
    ///
    /// - Fails if a podcast with the same slug already exists
    pub async fn create_feed(&self, feed: PodcastFeed) -> Result<PodcastFeed, Report<CreateError>> {
        trace!(podcast = %feed.podcast.slug, "Inserting new podcast and episodes");
        let tx = self.db.begin().await.change_context(CreateError::Begin)?;
        let podcast = insert_podcast(feed.podcast)
            .exec_with_returning(&tx)
            .await
            .map_err(|e| {
                let e2 = if let Some(SqlErr::UniqueConstraintViolation(_)) = e.sql_err() {
                    CreateError::AlreadyExists
                } else {
                    CreateError::Podcast
                };
                Report::new(e).change_context(e2)
            })?;

        let episodes = insert_episodes(feed.episodes, podcast.primary_key)
            .exec_with_returning(&tx)
            .await
            .change_context(CreateError::Episodes)?;
        tx.commit().await.change_context(CreateError::Commit)?;
        Ok(PodcastFeed { podcast, episodes })
    }
}

fn insert_podcast(podcast: PodcastInfo) -> Insert<podcast::ActiveModel> {
    let model = podcast::ActiveModel {
        primary_key: NotSet,
        ..podcast::ActiveModel::from(podcast)
    };
    podcast::Entity::insert(model)
}

pub(super) fn insert_episodes(
    episodes: Vec<EpisodeInfo>,
    podcast_key: PodcastKey,
) -> InsertMany<episode::ActiveModel> {
    let models = episodes.into_iter().map(|episode| episode::ActiveModel {
        primary_key: NotSet,
        podcast_key: Set(Some(podcast_key)),
        ..episode::ActiveModel::from(episode)
    });
    episode::Entity::insert_many(models)
}

/// Errors from [`MetadataRepository::create_feed`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CreateError {
    #[error("Unable to begin database transaction")]
    Begin,
    #[error("Podcast with this slug already exists")]
    AlreadyExists,
    #[error("Unable to insert podcast")]
    Podcast,
    #[error("Unable to insert episodes")]
    Episodes,
    #[error("Unable to commit database transaction")]
    Commit,
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn _insert_podcast() {
        // Arrange
        let feed = MetadataRepositoryExample::example_feeds()
            .into_iter()
            .next()
            .expect("should have at least one feed");

        // Act
        let statement = insert_podcast(feed.podcast).build(DB_BACKEND);

        // Assert
        assert_snapshot!(format_sql(&statement));
    }

    #[test]
    fn _insert_episodes() {
        // Arrange
        let feed = MetadataRepositoryExample::example_feeds()
            .into_iter()
            .next()
            .expect("should have at least one feed");

        // Act
        let statement = insert_episodes(feed.episodes, feed.podcast.primary_key).build(DB_BACKEND);

        // Assert
        assert_snapshot!(format_sql(&statement));
    }

    #[tokio::test]
    pub async fn create_feed() {
        // Arrange
        let metadata = MetadataRepositoryExample::create().await;
        let mut feed = MetadataRepositoryExample::example_feeds()
            .into_iter()
            .next()
            .expect("should have at least one feed");
        feed.podcast.slug = Slug::from_str("new-podcast").expect("valid slug");
        let _logger = init_test_logger();

        // Act
        let result = metadata.create_feed(feed.clone()).await;

        // Assert
        let saved_feed = result.assert_ok_debug();
        assert_eq!(saved_feed.podcast.slug, feed.podcast.slug);
    }

    #[tokio::test]
    pub async fn create_feed__already_exists() {
        // Arrange
        let metadata = MetadataRepositoryExample::create().await;
        let feed = MetadataRepositoryExample::example_feeds()
            .into_iter()
            .next()
            .expect("should have at least one feed");
        let _logger = init_test_logger();

        // Act
        let result = metadata.create_feed(feed).await;

        // Assert
        let err = result.expect_err("should fail when slug exists");
        assert_eq!(err.current_context(), &CreateError::AlreadyExists);
    }
}
