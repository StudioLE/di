use crate::prelude::*;

/// Downloads a single episode through a multi-step pipeline.
///
/// - Fetch audio file
/// - Fetch and resize artwork
/// - Add ID3 tags
/// - Save file paths to database
#[derive(Service)]
pub struct DownloadHandler {
    pub(super) paths: Arc<PathProvider>,
    pub(super) http: Arc<HttpClient>,
    pub(super) metadata: Arc<MetadataRepository>,
}

#[async_trait]
impl Execute<DownloadRequest, DownloadResponse, Report<DownloadError>> for DownloadHandler {
    /// Execute the download pipeline for a single episode.
    async fn execute(
        &self,
        request: &DownloadRequest,
    ) -> Result<DownloadResponse, Report<DownloadError>> {
        trace!(%request, "Retrieving podcast and episode from DB");
        let context = self.context_step(request).await?;
        let podcast = context.podcast.to_string();
        let episode = context.episode.to_string();
        if let Some(path) = &context.episode.file_sub_path {
            debug!(podcast, episode, %path, "Skipping already downloaded");
            return Ok(DownloadResponse {
                file_path: path.as_ref().clone(),
                image_path: context.episode.image_sub_path.as_deref().cloned(),
            });
        }
        trace!(podcast, episode, "Downloading episode file");
        self.download_file_step(&context).await?;
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
