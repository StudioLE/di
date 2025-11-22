use crate::prelude::*;

pub struct ScrapeCommand {
    pub(super) http: HttpClient,
    pub(super) metadata: MetadataRepository,
}

impl ScrapeCommand {
    #[must_use]
    pub fn new(http: HttpClient, metadata: MetadataRepository) -> Self {
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
        PodcastFromRss::execute(channel, &options.podcast_slug)
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
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");
        let command = ScrapeCommand::new(services.http, services.metadata);
        let options = ScrapeOptions {
            podcast_slug: "irl".to_owned(),
            url: Url::parse("https://irlpodcast.org").expect("URL should parse"),
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
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");
        let command = ScrapeCommand::new(services.http, services.metadata);
        let options = ScrapeOptions {
            podcast_slug: "irl-rss".to_owned(),
            url: Url::parse("https://feeds.simplecast.com/lP7owBq8").expect("URL should parse"),
        };

        // Act
        let result = command.execute(options).await;

        // Assert
        let podcast = result.assert_ok_debug();
        assert!(podcast.episodes.len() > 30);
    }
}
