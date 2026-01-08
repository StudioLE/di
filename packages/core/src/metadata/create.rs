use crate::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::*;

impl MetadataRepository {
    /// Save a the [`PodcastInfo`] and [`EpisodeInfo`] entities.
    ///
    /// If a podcast with the same slug already exists it will be overwritten.
    #[allow(clippy::single_match_else)]
    pub async fn save_feed(&self, feed: PodcastFeed) -> Result<PodcastFeed, Report<SaveError>> {
        let tx = self.db.begin().await.change_context(SaveError::Begin)?;
        let primary_key = match get_podcast_key_by_slug(&tx, &feed.podcast.slug).await? {
            Some(key) => {
                trace!(
                    podcast = %feed.podcast.slug,
                    key, "Overwriting existing podcast"
                );
                remove_podcast(&tx, key).await?;
                Set(key)
            }
            None => {
                trace!(podcast = %feed.podcast.slug, "Inserting new podcast");
                NotSet
            }
        };
        let model = podcast::ActiveModel {
            primary_key,
            ..podcast::ActiveModel::from(feed.podcast)
        };
        let podcast = insert_podcast(model)
            .exec_with_returning(&tx)
            .await
            .change_context(SaveError::Podcast)?;
        let models = feed
            .episodes
            .into_iter()
            .map(|episode| episode::ActiveModel {
                primary_key: NotSet,
                podcast_key: Set(Some(podcast.primary_key)),
                ..episode::ActiveModel::from(episode)
            });
        let episodes = insert_episodes(models)
            .exec_with_returning(&tx)
            .await
            .change_context(SaveError::Episodes)?;
        tx.commit().await.change_context(SaveError::Commit)?;
        Ok(PodcastFeed { podcast, episodes })
    }
}

fn remove_podcast_statement(primary_key: u32, backend: DatabaseBackend) -> Statement {
    podcast::Entity::delete_by_id(primary_key).build(backend)
}

async fn remove_podcast(
    tx: &DatabaseTransaction,
    primary_key: u32,
) -> Result<ExecResult, Report<SaveError>> {
    let backend = ConnectionTrait::get_database_backend(tx);
    let statement = remove_podcast_statement(primary_key, backend);
    tx.execute_raw(statement)
        .await
        .change_context(SaveError::Remove)
}

fn get_podcast_key_by_slug_select(slug: &Slug) -> Select<podcast::Entity> {
    podcast::Entity::find()
        .select_only()
        .column(podcast::Column::PrimaryKey)
        .filter(podcast::Column::Slug.eq(slug.as_str()))
}

async fn get_podcast_key_by_slug(
    tx: &DatabaseTransaction,
    slug: &Slug,
) -> Result<Option<u32>, Report<SaveError>> {
    get_podcast_key_by_slug_select(slug)
        .into_tuple::<u32>()
        .one(tx)
        .await
        .change_context(SaveError::Unique)
}

fn insert_podcast(model: podcast::ActiveModel) -> Insert<podcast::ActiveModel> {
    podcast::Entity::insert(model)
}

fn insert_episodes(
    models: impl IntoIterator<Item = episode::ActiveModel>,
) -> InsertMany<episode::ActiveModel> {
    episode::Entity::insert_many(models)
}

#[derive(Clone, Debug, Error)]
pub enum SaveError {
    #[error("Unable to begin database transaction")]
    Begin,
    #[error("Unable to check if podcast already exists")]
    Unique,
    #[error("Unable to remove previous podcast")]
    Remove,
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
    fn _get_podcast_key_by_slug_select() {
        // Arrange
        let slug = MetadataRepositoryExample::podcast_slug();

        // Act
        let statement = get_podcast_key_by_slug_select(&slug).build(DB_BACKEND);

        // Assert
        assert_snapshot!(format_sql(&statement));
    }

    #[test]
    fn _remove_podcast_statement() {
        // Arrange
        // Act
        let statement = remove_podcast_statement(PODCAST_KEY, DB_BACKEND);

        // Assert
        assert_snapshot!(format_sql(&statement));
    }

    #[test]
    fn _insert_podcast() {
        // Arrange
        let feed = MetadataRepositoryExample::example_feeds()
            .into_iter()
            .next()
            .expect("should have at least one feed");
        let model = podcast::ActiveModel::from(feed.podcast);

        // Act
        let statement = insert_podcast(model).build(DB_BACKEND);

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
        let models = feed.episodes.into_iter().map(|e| episode::ActiveModel {
            primary_key: NotSet,
            podcast_key: Set(Some(PODCAST_KEY)),
            ..episode::ActiveModel::from(e)
        });

        // Act
        let statement = insert_episodes(models).build(DB_BACKEND);

        // Assert
        assert_snapshot!(format_sql(&statement));
    }

    #[tokio::test]
    pub async fn save_feed() {
        // Arrange
        let metadata = MetadataRepositoryExample::create().await;
        let feeds = MetadataRepositoryExample::example_feeds();
        let feed = feeds.first().expect("should be at least one feed").clone();
        let slug = feed.podcast.slug.clone();
        let _logger = init_test_logger();

        // Act
        let result = metadata.save_feed(feed).await;

        // Assert
        let saved_feed = result.assert_ok_debug();
        assert_eq!(saved_feed.podcast.slug, slug);
        assert_eq!(saved_feed.podcast.primary_key, 1, "podcast primary key");
        let episode = saved_feed
            .episodes
            .iter()
            .find(|episode| episode.episode == Some(1) && episode.season == Some(1))
            .expect("episode should exist");
        assert_eq!(
            episode.primary_key, 55,
            "episode primary key should have changed"
        );
    }
}
