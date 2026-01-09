use crate::prelude::*;
use sea_orm::*;

impl FetchHandler {
    /// Get the feed URL for a podcast by its slug.
    pub(super) async fn get_feed_url(&self, slug: &Slug) -> Result<UrlWrapper, Report<FetchError>> {
        self.get_feed_url_internal(slug)
            .await
            .attach(format!("Podcast: {slug}"))
    }

    async fn get_feed_url_internal(&self, slug: &Slug) -> Result<UrlWrapper, Report<FetchError>> {
        self.metadata
            .get_feed_url(slug)
            .await
            .change_context(FetchError::Repository)?
            .ok_or(FetchError::NoPodcast)?
            .ok_or(Report::new(FetchError::NoFeedUrl))
    }
}

impl MetadataRepository {
    /// Get the feed URL for a podcast by its slug.
    ///
    /// - Returns `Ok(None)` if the podcast does not exist
    /// - Returns `Ok(Some(None))` if the podcast exists but has no feed URL
    async fn get_feed_url(&self, slug: &Slug) -> Result<Option<Option<UrlWrapper>>, DbErr> {
        self.get_feed_url_select(slug)
            .into_tuple::<Option<UrlWrapper>>()
            .one(&self.db)
            .await
    }

    #[allow(clippy::unused_self)]
    fn get_feed_url_select(&self, slug: &Slug) -> Select<podcast::Entity> {
        podcast::Entity::find()
            .select_only()
            .column(podcast::Column::FeedUrl)
            .filter(podcast::Column::Slug.eq(slug.to_string()))
    }

    #[cfg(test)]
    fn get_feed_url_query(&self, slug: &Slug) -> Statement {
        self.get_feed_url_select(slug)
            .build(self.db.get_database_backend())
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[tokio::test]
    pub async fn get_feed_url_query() {
        // Arrange
        let metadata = MetadataRepositoryExample::create().await;
        let slug = MetadataRepositoryExample::podcast_slug();

        // Act
        let statement = metadata.get_feed_url_query(&slug);

        // Assert
        let sql = format_sql(&statement);
        assert_snapshot!(sql);
    }

    #[tokio::test]
    pub async fn get_feed_url__not_found() {
        // Arrange
        let metadata = MetadataRepositoryExample::create().await;
        let slug = Slug::from_str("non-existent").expect("should be valid slug");

        // Act
        let result = metadata.get_feed_url(&slug).await;

        // Assert
        let option = result.assert_ok_debug();
        assert!(
            option.is_none(),
            "should return None for non-existent podcast"
        );
    }

    #[tokio::test]
    pub async fn get_feed_url__no_url() {
        // Arrange
        let metadata = MetadataRepositoryExample::create().await;
        let slug = MetadataRepositoryExample::podcast_slug();

        // Act
        let result = metadata.get_feed_url(&slug).await;

        // Assert
        let option = result.assert_ok_debug();
        assert_eq!(
            option,
            Some(None),
            "should return Some(None) for podcast without feed URL"
        );
    }
}
