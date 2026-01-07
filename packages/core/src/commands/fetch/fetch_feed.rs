use super::podcast_from_rss::PodcastFromRss;
use crate::prelude::*;
use rss::Channel as RssChannel;

impl FetchHandler {
    /// Fetch and parse a podcast feed without saving.
    ///
    /// - Resolves Simplecast URLs to RSS feeds
    /// - Fetches and parses the RSS feed
    pub async fn fetch_feed(
        &self,
        slug: &Slug,
        url: &UrlWrapper,
    ) -> Result<PodcastFeed, Report<FetchRssError>> {
        let content_type = self
            .http
            .head(url)
            .await
            .change_context(FetchRssError::Xml)?;
        let resolved_url = match content_type.as_str() {
            "application/xml" | "text/xml" => url.clone(),
            _ => self
                .get_simplecast_rss(slug, url)
                .await
                .change_context(FetchRssError::Convert)?,
        };
        self.parse_rss(&resolved_url, slug).await
    }

    async fn parse_rss(
        &self,
        url: &UrlWrapper,
        slug: &Slug,
    ) -> Result<PodcastFeed, Report<FetchRssError>> {
        let path = self
            .http
            .get(url, Some(RSS_EXTENSION))
            .await
            .change_context(FetchRssError::Xml)?;
        let file = File::open(&path)
            .change_context(FetchRssError::Open)
            .attach_path(path)?;
        let reader = BufReader::new(file);
        let channel = RssChannel::read_from(reader).change_context(FetchRssError::Parse)?;
        PodcastFromRss::execute(channel, slug.clone()).change_context(FetchRssError::Convert)
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[tokio::test]
    #[serial]
    #[allow(deprecated)]
    pub async fn fetch_feed_simplecast() {
        // Arrange
        let _logger = init_test_logger();
        let metadata = MetadataRepositoryExample::create().await;
        let handler = ServiceProvider::new()
            .with_instance(metadata)
            .get_service::<FetchHandler>()
            .await
            .expect("should be able to get handler");

        // Act
        let result = handler
            .fetch_feed(&example_slug(), &example_simplecast_url())
            .await;

        // Assert
        let podcast = result.assert_ok_debug();
        assert!(podcast.episodes.len() > 30);
    }

    #[tokio::test]
    #[serial]
    #[allow(deprecated)]
    pub async fn fetch_feed_rss() {
        // Arrange
        let metadata = MetadataRepositoryExample::create().await;
        let handler = ServiceProvider::new()
            .with_instance(metadata)
            .get_service::<FetchHandler>()
            .await
            .expect("should be able to get handler");
        let _logger = init_test_logger();

        // Act
        let result = handler
            .fetch_feed(&example_slug(), &example_rss_url())
            .await;

        // Assert
        let podcast = result.assert_ok_debug();
        assert!(podcast.episodes.len() > 30);
    }
}
