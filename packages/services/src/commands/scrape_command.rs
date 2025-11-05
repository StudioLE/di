use crate::prelude::*;

pub struct ScrapeCommand {
    pub(super) http: HttpClient,
    pub(super) metadata: MetadataStore,
}

impl ScrapeCommand {
    #[must_use]
    pub fn new(http: HttpClient, metadata: MetadataStore) -> Self {
        Self { http, metadata }
    }

    pub async fn execute(&self, options: ScrapeOptions) -> Result<Podcast, Report<ScrapeError>> {
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
        let podcast = self
            .execute_rss(&options)
            .await
            .change_context(ScrapeError::Rss)?;
        info!("Fetched {} episodes", podcast.episodes.len());
        self.metadata
            .put(&podcast)
            .change_context(ScrapeError::Save)?;
        Ok(podcast)
    }

    pub(super) async fn execute_rss(
        &self,
        options: &ScrapeOptions,
    ) -> Result<Podcast, Report<ScrapeRssError>> {
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
        let mut podcast: Podcast = channel.try_into().change_context(ScrapeRssError::Convert)?;
        podcast.id.clone_from(&options.podcast_id);
        Ok(podcast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[traced_test]
    pub async fn scrape_command_simplecast() {
        // Arrange
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");
        let command = ScrapeCommand::new(services.http, services.metadata);
        let options = ScrapeOptions {
            podcast_id: "irl".to_owned(),
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
    pub async fn scrape_command_rss() {
        // Arrange
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");
        let command = ScrapeCommand::new(services.http, services.metadata);
        let options = ScrapeOptions {
            podcast_id: "irl-rss".to_owned(),
            url: Url::parse("https://feeds.simplecast.com/lP7owBq8").expect("URL should parse"),
        };

        // Act
        let result = command.execute(options).await;

        // Assert
        let podcast = result.assert_ok_debug();
        assert!(podcast.episodes.len() > 30);
    }
}
