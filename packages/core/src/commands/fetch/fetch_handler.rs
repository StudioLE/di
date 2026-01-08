use crate::prelude::*;

/// Fetches an existing podcast using its stored feed URL.
///
/// - Reads the feed URL from the database
/// - Fetches and parses the RSS feed
/// - Saves the updated podcast and episodes to the database
#[derive(Clone)]
pub struct FetchHandler {
    pub(super) http: Arc<HttpClient>,
    pub(super) metadata: Arc<MetadataRepository>,
}

impl Service for FetchHandler {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self {
            http: services.get_service().await?,
            metadata: services.get_service().await?,
        })
    }
}

#[async_trait]
impl Execute<FetchRequest, FetchResponse, Report<FetchError>> for FetchHandler {
    /// Execute the fetch handler.
    async fn execute(&self, request: &FetchRequest) -> Result<FetchResponse, Report<FetchError>> {
        trace!(slug = %request.slug, "Getting feed URL");
        let feed_url = self.get_feed_url(&request.slug).await?;
        trace!(slug = %request.slug, "Fetching feed");
        let mut feed = self
            .fetch_feed(&request.slug, &feed_url)
            .await
            .change_context(FetchError::Rss)?;
        trace!(slug = %request.slug, episodes = feed.episodes.len(), "Fetched feed");
        feed.podcast.feed_url = Some(feed_url);
        let feed = self
            .metadata
            .update_feed(feed)
            .await
            .change_context(FetchError::Save)?;
        info!(slug = %feed.podcast.slug, episodes = feed.episodes.len(), "Fetched podcast");
        Ok(FetchResponse {
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
    pub async fn fetch_handler() {
        // Arrange
        let services = TestServiceProvider::create().await;
        let add_handler = services
            .get_service::<AddHandler>()
            .await
            .expect("should be able to get add handler");
        let add_request = AddRequest {
            slug: example_slug(),
            feed_url: example_rss_url(),
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
            slug: example_slug(),
        };
        let _logger = init_test_logger();

        // Act
        let result = handler.execute(&request).await;

        // Assert
        let response = result.assert_ok_debug();
        assert!(response.episode_count > 0);
    }
}
