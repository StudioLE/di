use crate::prelude::*;

const BANNER_WIDTH: u32 = 960;
const BANNER_HEIGHT: u32 = 540;
const COVER_SIZE: u32 = 720;

pub struct CoverCommand {
    paths: Arc<PathProvider>,
    http: Arc<HttpClient>,
    metadata: Arc<MetadataRepository>,
}

impl Service for CoverCommand {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<ServiceError>> {
        Ok(Self::new(
            services.get_service().await?,
            services.get_service().await?,
            services.get_service().await?,
        ))
    }
}

impl CoverCommand {
    #[must_use]
    pub fn new(
        paths: Arc<PathProvider>,
        http: Arc<HttpClient>,
        metadata: Arc<MetadataRepository>,
    ) -> Self {
        Self {
            paths,
            http,
            metadata,
        }
    }

    pub async fn execute(&self, options: CoverOptions) -> Result<(), Report<CoverError>> {
        let feed = self
            .metadata
            .get_feed_by_slug(options.podcast_slug.clone(), None)
            .await
            .change_context(CoverError::Repository)?
            .ok_or(CoverError::NoPodcast)?;
        let url = feed
            .podcast
            .image
            .clone()
            .map(Url::from)
            .ok_or(CoverError::NoImage)?;
        let src = self
            .http
            .get(&url, None)
            .await
            .change_context(CoverError::GetImage)
            .attach_url(&url)?;
        let banner = self.paths.get_banner_path(&options.podcast_slug);
        let cover = self.paths.get_cover_path(&options.podcast_slug);
        create_parent_dir_if_not_exist(&banner)
            .await
            .change_context(CoverError::CreateDirectory)?;
        let resize = Resize::new(&src)
            .change_context(CoverError::CreateImage)
            .attach_path(src)?;
        let banner = resize
            .to_file(&banner, BANNER_WIDTH, BANNER_HEIGHT)
            .change_context(CoverError::CreateImage)?;
        let cover = resize
            .to_file(&cover, COVER_SIZE, COVER_SIZE)
            .change_context(CoverError::CreateImage)?;
        info!("Created images");
        trace!(banner = %banner.display(), cover = %cover.display(), "Created images");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[traced_test]
    pub async fn cover_command() {
        // Arrange
        let services = ServiceProvider::new();
        let command = services
            .get_service::<CoverCommand>()
            .await
            .expect("should be able to get command");
        let options = CoverOptions {
            podcast_slug: example_slug(),
        };

        // Act
        let result = command.execute(options).await;

        // Assert
        result.assert_ok_debug();
    }
}
