use super::podcast_from_rss::PodcastFromRss;
use crate::prelude::*;
use rss::Channel as RssChannel;

/// Maximum number of feed redirects to follow.
const MAX_FEED_REDIRECTS: usize = 10;

impl FetchHandler {
    /// Fetch and parse a podcast feed without saving.
    ///
    /// - Follows `new_feed_url` redirects (up to 10)
    /// - Sets `feed_url` to the canonical URL
    /// - Resolves Simplecast URLs to RSS feeds
    pub async fn fetch_feed(
        &self,
        slug: &Slug,
        url: &UrlWrapper,
    ) -> Result<PodcastFeed, Report<FetchError>> {
        let (mut feed, url) = self.fetch_feed_with_redirect(slug, url).await?;
        feed.podcast.feed_url = Some(url);
        Ok(feed)
    }

    async fn fetch_feed_with_redirect(
        &self,
        slug: &Slug,
        url: &UrlWrapper,
    ) -> Result<(PodcastFeed, UrlWrapper), Report<FetchError>> {
        let mut visited = HashSet::new();
        let mut current = url.clone();
        let mut i = 0;
        loop {
            trace!(url = %current, "Fetching feed");
            let feed = self
                .fetch_feed_without_redirect(slug, &current)
                .await
                .change_context(FetchError::Rss)?;
            visited.insert(current.clone());
            let Some(next) = &feed.podcast.new_feed_url else {
                return Ok((feed, current));
            };
            if next == &current {
                return Ok((feed, current));
            }
            if visited.contains(next) {
                return Err(Report::new(FetchError::RedirectLoop).attach(format!("URL: {next}")));
            }
            if i >= MAX_FEED_REDIRECTS {
                return Err(Report::new(FetchError::TooManyRedirects)
                    .attach(format!("Limit: {MAX_FEED_REDIRECTS}")));
            }
            trace!(%current, new_feed_url = %next, redirects = i, "Feed includes a `new_feed_url`");
            current = next.clone();
            i += 1;
        }
    }

    async fn fetch_feed_without_redirect(
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
