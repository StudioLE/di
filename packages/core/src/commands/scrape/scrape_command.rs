use super::from_rss::PodcastFromRss;
use crate::prelude::*;
use rss::Channel as RssChannel;

pub struct ScrapeCommand {
    pub(super) http: Arc<HttpClient>,
    pub(super) metadata: Arc<MetadataRepository>,
}

impl Service for ScrapeCommand {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self::new(
            services.get_service().await?,
            services.get_service().await?,
        ))
    }
}

impl ScrapeCommand {
    #[must_use]
    pub fn new(http: Arc<HttpClient>, metadata: Arc<MetadataRepository>) -> Self {
        Self { http, metadata }
    }

    pub async fn execute(
        &self,
        options: ScrapeOptions,
    ) -> Result<PodcastFeed, Report<ScrapeError>> {
        let content_type = self
            .http
            .head(&options.url)
            .await
            .change_context(ScrapeError::Head)?;
        let mut options = options.clone();
        match content_type.as_str() {
            "application/xml" | "text/xml" => {}
            _ => {
                options.url = self
                    .get_simplecast_rss(&options)
                    .await
                    .change_context(ScrapeError::Simplecast)?;
            }
        }
        let feed = self
            .execute_rss(&options)
            .await
            .change_context(ScrapeError::Rss)?;
        info!("Fetched {} episodes", feed.episodes.len());
        let feed = self
            .metadata
            .save_feed(feed)
            .await
            .change_context(ScrapeError::Save)?;
        Ok(feed)
    }

    pub(super) async fn execute_rss(
        &self,
        options: &ScrapeOptions,
    ) -> Result<PodcastFeed, Report<ScrapeRssError>> {
        let path = self
            .http
            .get(&options.url, Some(RSS_EXTENSION))
            .await
            .change_context(ScrapeRssError::Xml)?;
        let file = File::open(&path)
            .change_context(ScrapeRssError::Open)
            .attach_path(path)?;
        let reader = BufReader::new(file);
        let channel = RssChannel::read_from(reader).change_context(ScrapeRssError::Parse)?;
        PodcastFromRss::execute(channel, options.podcast_slug.clone())
            .change_context(ScrapeRssError::Convert)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[traced_test]
    #[serial]
    pub async fn scrape_command_simplecast() {
        // Arrange
        let mut services = ServiceProvider::new();
        let metadata = MetadataRepositoryExample::create().await;
        services.add_instance(metadata);
        let command = services
            .get_service::<ScrapeCommand>()
            .await
            .expect("should be able to get command");
        let options = ScrapeOptions {
            podcast_slug: example_slug(),
            url: example_simplecast_url(),
        };

        // Act
        let result = command.execute(options).await;

        // Assert
        let podcast = result.assert_ok_debug();
        assert!(podcast.episodes.len() > 30);
    }

    #[tokio::test]
    #[traced_test]
    #[serial]
    pub async fn scrape_command_rss() {
        // Arrange
        let mut services = ServiceProvider::new();
        let metadata = MetadataRepositoryExample::create().await;
        services.add_instance(metadata);
        let command = services
            .get_service::<ScrapeCommand>()
            .await
            .expect("should be able to get command");
        let options = ScrapeOptions {
            podcast_slug: example_slug(),
            url: example_rss_url(),
        };

        // Act
        let result = command.execute(options).await;

        // Assert
        let podcast = result.assert_ok_debug();
        assert!(podcast.episodes.len() > 30);
    }
}
