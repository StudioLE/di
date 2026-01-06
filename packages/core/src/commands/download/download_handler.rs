use crate::prelude::*;

pub struct DownloadHandler {
    pub(super) paths: Arc<PathProvider>,
    pub(super) http: Arc<HttpClient>,
    pub(super) metadata: Arc<MetadataRepository>,
}

impl Service for DownloadHandler {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self {
            paths: services.get_service().await?,
            http: services.get_service().await?,
            metadata: services.get_service().await?,
        })
    }
}

#[async_trait]
impl Execute<DownloadRequest, DownloadResponse, Report<DownloadError>> for DownloadHandler {
    async fn execute(
        &self,
        request: &DownloadRequest,
    ) -> Result<DownloadResponse, Report<DownloadError>> {
        trace!(%request, "Retrieving podcast and episode from DB");
        let context = self.context_step(request).await?;
        let podcast = context.podcast.to_string();
        let episode = context.episode.to_string();
        trace!(podcast, episode, "Downloading episode file");
        self.download_episode_step(&context).await?;
        trace!(podcast, episode, "Downloading episode image");
        self.download_image_step(&context).await?;
        trace!(podcast, episode, "Resizing episode image");
        self.resize_step(&context).await?;
        trace!(podcast, episode, "Tagging episode");
        self.tag_step(&context)?;
        trace!(podcast, episode, "Saving episode");
        self.save_step(&context).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[serial]
    pub async fn download_handler() {
        // Arrange
        let services = TestServiceProvider::create().await;
        let download = services
            .get_service::<DownloadHandler>()
            .await
            .expect("should be able to get command");
        let request = DownloadRequest::new(PODCAST_KEY, EPISODE_KEY);
        let _logger = init_test_logger();

        // Act
        let result = download.execute(&request).await;

        // Assert
        result.assert_ok_debug();
    }
}
