use crate::prelude::*;

impl DownloadHandler {
    pub(super) async fn download_episode_step(
        &self,
        context: &DownloadContext,
    ) -> Result<(), Report<DownloadError>> {
        let url = context.episode.source_url.as_ref();
        self.http
            .download(url, context.file_path.clone())
            .await
            .change_context(DownloadError::DownloadEpisode)
    }

    pub(super) async fn download_image_step(
        &self,
        context: &DownloadContext,
    ) -> Result<(), Report<DownloadError>> {
        let Some(url) = &context.episode.image else {
            return Ok(());
        };
        let Some(path) = &context.image_path else {
            return Ok(());
        };
        self.http
            .download(url.as_ref(), path.clone())
            .await
            .change_context(DownloadError::DownloadImage)
    }
}
