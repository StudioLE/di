use crate::prelude::*;

impl DownloadHandler {
    /// Download the episode audio file to the local filesystem.
    pub(super) async fn download_episode_step(
        &self,
        context: &DownloadContext,
    ) -> Result<(), Report<DownloadError>> {
        let hardlink = self.paths.get_hard_link_from_cache();
        self.http
            .download(
                &context.episode.source_url,
                context.file_path.clone(),
                hardlink,
            )
            .await
            .change_context(DownloadError::DownloadEpisode)
    }

    /// Download the episode artwork if available.
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
        let hardlink = self.paths.get_hard_link_from_cache();
        self.http
            .download(url, path.clone(), hardlink)
            .await
            .change_context(DownloadError::DownloadImage)
    }
}
