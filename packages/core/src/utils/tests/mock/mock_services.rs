use crate::prelude::*;

/// Factory for creating mock [`ServiceProvider`] instances.
pub struct MockServices {
    /// Mock feeds to insert into the database.
    metadata: Option<MockFeedsFactory>,
    /// Whether to prime the HTTP cache with mock RSS data.
    rss_feed: bool,
}

impl MockServices {
    #[must_use]
    pub fn rss_url() -> UrlWrapper {
        UrlWrapper::from_str("https://example.com/mock-feed.xml").expect("URL should parse")
    }

    /// Create a new [`MockServices`] instance with no mock feeds.
    #[must_use]
    pub fn new() -> Self {
        Self {
            metadata: None,
            rss_feed: false,
        }
    }

    /// Prime the HTTP cache with mock RSS feed data.
    #[must_use]
    pub fn with_metadata_factory(mut self, factory: MockFeedsFactory) -> Self {
        self.metadata = Some(factory);
        self
    }

    /// Prime the HTTP cache with mock RSS feed data.
    #[must_use]
    pub fn with_metadata(mut self) -> Self {
        self.metadata = Some(MockFeedsFactory::default());
        self
    }

    /// Prime the HTTP cache with mock RSS feed data.
    #[must_use]
    pub fn with_rss_feed(mut self) -> Self {
        self.rss_feed = true;
        self
    }

    /// Create a new [`ServiceProvider`] instance with mock services.
    ///
    /// - Creates a temporary data directory and stores it in app options
    /// - Creates a temporary sqlite database
    /// - Inserts mock feeds into the database.
    pub async fn create(self) -> ServiceProvider {
        trace!("Creating mock services");
        let options = mock_app_options();
        create_data_dir(&options).await;
        let services = ServiceProvider::new()
            .with_instance(options)
            .with_commands()
            .await
            .expect("should be able to create services with commands");
        if let Some(factory) = self.metadata {
            insert_db_feeds(&services, factory).await;
        } else {
            debug!("No mock feeds. Database will be empty");
        }
        if self.rss_feed {
            write_mock_rss_feed(&services).await;
        }
        services
    }
}

async fn create_data_dir(options: &AppOptions) {
    let data_dir = options.data_dir.as_ref().expect("data dir should be set");
    trace!(path = %data_dir.display(), "Creating temp data dir");
    create_dir_all(&data_dir)
        .await
        .expect("should be able to create temp data dir");
}

async fn insert_db_feeds(services: &ServiceProvider, factory: MockFeedsFactory) {
    trace!(podcasts = ?factory.podcast_count, "Inserting mock feeds to database");
    let metadata = services
        .get_service::<MetadataRepository>()
        .await
        .expect("should be able to get MetadataRepository");
    let mock = factory.create();
    for feed in mock.feeds {
        metadata
            .create_feed(feed)
            .await
            .expect("should be able to save feed");
    }
}

async fn write_mock_rss_feed(services: &ServiceProvider) {
    trace!("Writing mock rss feed to HttpCache");
    let cache = services
        .get_service::<HttpCache>()
        .await
        .expect("should be able to get HttpCache");
    let factory = MockFeedsFactory {
        podcast_count: 1,
        ..MockFeedsFactory::default()
    };
    let feed = factory.create().feeds.pop().expect("should have one feed");
    let channel = PodcastToRss::execute(feed);
    let xml = channel.to_string();
    let url = MockServices::rss_url();
    cache
        .write_string(&url, Some("head"), "application/xml")
        .await
        .expect("should write head");
    cache
        .write_string(&url, Some(RSS_EXTENSION), &xml)
        .await
        .expect("should write rss");
}

impl Default for MockServices {
    /// Create a new [`MockServices`] instance with mock feeds.
    fn default() -> Self {
        MockServices::new().with_metadata()
    }
}

fn mock_app_options() -> AppOptions {
    let temp_dir = TempDirectory::default()
        .create()
        .expect("should be able to create temp dir");
    let data_dir = temp_dir.join("data");
    AppOptions {
        data_dir: Some(data_dir),
        ..AppOptions::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn service_provider_create() {
        // Arrange
        let services = MockServices::default().create().await;

        // Act
        let options = services
            .get_service::<AppOptions>()
            .await
            .expect("should be able to get options");

        // Assert
        let data_dir = options
            .as_ref()
            .clone()
            .data_dir
            .expect("should have data dir");
        assert!(data_dir.exists());
        assert!(data_dir.components().any(|component| {
            let component = component.as_os_str().to_str().unwrap_or_default();
            component.contains("service_provider_create")
        }));
    }
}
