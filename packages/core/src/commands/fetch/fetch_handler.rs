use crate::prelude::*;

/// Fetches an existing podcast using its stored feed URL.
///
/// - Reads the feed URL from the database
/// - Fetches and parses the RSS feed
/// - Saves the updated podcast and episodes to the database
#[derive(Clone, Service)]
pub struct FetchHandler {
    pub(super) http: Arc<HttpClient>,
    pub(super) metadata: Arc<MetadataRepository>,
}

#[async_trait]
impl Execute<FetchRequest, FetchResponse, Report<FetchError>> for FetchHandler {
    /// Execute the fetch handler.
    async fn execute(&self, request: &FetchRequest) -> Result<FetchResponse, Report<FetchError>> {
        trace!(slug = %request.slug, "Getting feed URL");
        let stored_url = self.get_feed_url(&request.slug).await?;
        trace!(slug = %request.slug, "Fetching feed");
        let feed = self.fetch_feed(&request.slug, &stored_url).await?;
        trace!(slug = %request.slug, episodes = feed.episodes.len(), "Fetched feed");
        let response = self
            .metadata
            .update_feed(feed)
            .await
            .change_context(FetchError::Save)?;
        info!(
            response.podcast_key,
            response.episodes_updated, response.episodes_inserted, "Fetched podcast"
        );
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[serial]
    pub async fn fetch_handler() {
        // Arrange
        let services = MockServices::new().with_rss_feed().create().await;
        let add_handler = services
            .get_service::<AddHandler>()
            .await
            .expect("should be able to get add handler");
        let add_request = AddRequest {
            slug: MockFeeds::podcast_slug(),
            feed_url: MockServices::rss_url(),
        };
        add_handler
            .execute(&add_request)
            .await
            .expect("should be able to add podcast");
        let handler = services
            .get_service::<FetchHandler>()
            .await
            .expect("should be able to get fetch handler");
        let request = FetchRequest {
            slug: MockFeeds::podcast_slug(),
        };
        let _logger = init_test_logger();

        // Act
        let result = handler.execute(&request).await;

        // Assert
        let response = result.assert_ok_debug();
        assert!(response.episodes_updated > 0, "updated");
    }
}
