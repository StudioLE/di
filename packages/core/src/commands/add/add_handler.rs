use crate::prelude::*;

/// Adds a new podcast from an RSS feed.
///
/// - Fetches and parses the RSS feed
/// - Stores the feed URL for future fetching
/// - Saves the podcast and episodes to the database
#[derive(Service)]
pub struct AddHandler {
    fetch: Arc<FetchHandler>,
    metadata: Arc<MetadataRepository>,
}

#[async_trait]
impl Execute<AddRequest, AddResponse, Report<AddError>> for AddHandler {
    /// Execute the add handler.
    async fn execute(&self, request: &AddRequest) -> Result<AddResponse, Report<AddError>> {
        trace!(slug = %request.slug, url = %request.feed_url, "Fetching podcast");
        let mut feed = self
            .fetch
            .fetch_feed(&request.slug, &request.feed_url)
            .await
            .change_context(AddError::Parse)?;
        trace!(slug = %request.slug, episodes = feed.episodes.len(), "Fetched feed");
        feed.podcast.feed_url = Some(request.feed_url.clone());
        let feed = self
            .metadata
            .create_feed(feed)
            .await
            .change_context(AddError::Save)?;
        info!(slug = %feed.podcast.slug, episodes = feed.episodes.len(), "Added podcast");
        Ok(AddResponse {
            podcast_key: feed.podcast.primary_key,
            episode_count: feed.episodes.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[serial]
    #[allow(deprecated)]
    pub async fn add_handler() {
        // Arrange
        let services = TestServiceProvider::create().await;
        let handler = services
            .get_service::<AddHandler>()
            .await
            .expect("should be able to get handler");
        let request = AddRequest {
            slug: example_slug(),
            feed_url: example_rss_url(),
        };
        let _logger = init_test_logger();

        // Act
        let result = handler.execute(&request).await;

        // Assert
        let response = result.assert_ok_debug();
        assert!(response.episode_count > 0);
    }
}
