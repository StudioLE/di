use crate::metadata::create_feed::insert_episodes;
use crate::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::*;

impl MetadataRepository {
    /// Update an existing podcast feed.
    ///
    /// - Fails if the podcast doesn't exist
    /// - Updates the podcast and replaces all episodes
    pub async fn update_feed(&self, feed: PodcastFeed) -> Result<PodcastFeed, Report<UpdateError>> {
        trace!(podcast = %feed.podcast.slug, "Updating existing podcast and replacing episodes");
        let tx = self.db.begin().await.change_context(UpdateError::Begin)?;
        let key = get_podcast_key_by_slug(&tx, &feed.podcast.slug)
            .await
            .change_context(UpdateError::CheckExists)?
            .ok_or(UpdateError::NotFound)?;
        remove_episodes(&tx, key)
            .await
            .change_context(UpdateError::RemoveEpisodes)?;
        let podcast = update_podcast(&tx, feed.podcast, key)
            .await
            .change_context(UpdateError::Podcast)?;
        let episodes = insert_episodes(feed.episodes, podcast.primary_key)
            .exec_with_returning(&tx)
            .await
            .change_context(UpdateError::Episodes)?;
        tx.commit().await.change_context(UpdateError::Commit)?;
        Ok(PodcastFeed { podcast, episodes })
    }
}

fn update_podcast_query(
    podcast: PodcastInfo,
    primary_key: PodcastKey,
) -> UpdateOne<podcast::ActiveModel> {
    let model = podcast::ActiveModel {
        primary_key: Unchanged(primary_key),
        slug: Unchanged(podcast.slug),
        feed_url: Set(podcast.feed_url),
        title: Set(podcast.title),
        description: Set(podcast.description),
        image: Set(podcast.image),
        language: Set(podcast.language),
        categories: Set(podcast.categories),
        explicit: Set(podcast.explicit),
        author: Set(podcast.author),
        link: Set(podcast.link),
        kind: Set(podcast.kind),
        copyright: Set(podcast.copyright),
        new_feed_url: Set(podcast.new_feed_url),
        generator: Set(podcast.generator),
    };
    podcast::Entity::update(model)
}

async fn update_podcast(
    tx: &DatabaseTransaction,
    podcast: PodcastInfo,
    primary_key: PodcastKey,
) -> Result<podcast::Model, DbErr> {
    update_podcast_query(podcast, primary_key).exec(tx).await
}

fn remove_episodes_query(podcast_key: PodcastKey) -> DeleteMany<episode::Entity> {
    episode::Entity::delete_many().filter(episode::Column::PodcastKey.eq(podcast_key))
}

async fn remove_episodes(tx: &DatabaseTransaction, podcast_key: PodcastKey) -> Result<(), DbErr> {
    remove_episodes_query(podcast_key).exec(tx).await?;
    Ok(())
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
) -> Result<Option<u32>, DbErr> {
    get_podcast_key_by_slug_select(slug)
        .into_tuple::<u32>()
        .one(tx)
        .await
}

/// Errors from [`MetadataRepository::update_feed`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum UpdateError {
    #[error("Unable to begin database transaction")]
    Begin,
    #[error("Unable to check if podcast exists")]
    CheckExists,
    #[error("Podcast with this slug does not exist")]
    NotFound,
    #[error("Unable to remove previous episodes")]
    RemoveEpisodes,
    #[error("Unable to update podcast")]
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
    fn _update_podcast_query() {
        // Arrange
        let feed = MetadataRepositoryExample::example_feeds()
            .into_iter()
            .next()
            .expect("should have at least one feed");

        // Act
        let statement = update_podcast_query(feed.podcast, PODCAST_KEY)
            .validate()
            .expect("query should be valid")
            .build(DB_BACKEND);

        // Assert
        assert_snapshot!(format_sql(&statement));
    }

    #[test]
    fn _remove_episodes_query() {
        // Arrange
        // Act
        let statement = remove_episodes_query(PODCAST_KEY).build(DB_BACKEND);

        // Assert
        assert_snapshot!(format_sql(&statement));
    }

    #[tokio::test]
    pub async fn update_feed() {
        // Arrange
        let metadata = MetadataRepositoryExample::create().await;
        let mut feed = MetadataRepositoryExample::example_feeds()
            .into_iter()
            .next()
            .expect("should have at least one feed");
        feed.podcast.title = "Updated Title".to_owned();
        let slug = feed.podcast.slug.clone();
        let _logger = init_test_logger();

        // Act
        let result = metadata.update_feed(feed).await;

        // Assert
        let saved_feed = result.assert_ok_debug();
        assert_eq!(saved_feed.podcast.slug, slug);
        assert_eq!(saved_feed.podcast.title, "Updated Title");
        assert_eq!(
            saved_feed.podcast.primary_key, 1,
            "should reuse primary key"
        );
    }

    #[tokio::test]
    pub async fn update_feed__not_found() {
        // Arrange
        let metadata = MetadataRepositoryExample::create().await;
        let mut feed = MetadataRepositoryExample::example_feeds()
            .into_iter()
            .next()
            .expect("should have at least one feed");
        feed.podcast.slug = Slug::from_str("non-existent").expect("valid slug");
        let _logger = init_test_logger();

        // Act
        let result = metadata.update_feed(feed).await;

        // Assert
        let err = result.expect_err("should fail when slug doesn't exist");
        assert_eq!(err.current_context(), &UpdateError::NotFound);
    }
}
