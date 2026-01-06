use crate::prelude::*;
use dirs::{cache_dir, data_dir};
use std::fs::create_dir;

const HTTP_DIR: &str = "http";
const PODCASTS_DIR: &str = "podcasts";
const RSS_FILE_NAME: &str = "feed.rss";
pub const METADATA_DB: &str = "metadata.db";
const BANNER_FILE_NAME: &str = "banner.jpg";
const COVER_FILE_NAME: &str = "cover.jpg";

/// Service for providing file paths and URL.
#[derive(Clone)]
pub struct PathProvider {
    /// Directory for app data.
    ///
    /// Default: `$HOME/.local/share/alnwick` (or equivalent)
    data_dir: PathBuf,
    /// Directory for app cache.
    ///
    /// Default: `$HOME/.cache/alnwick` (or equivalent)
    cache_dir: PathBuf,
    /// Should hardlinks be used when copying between cache and data directory?
    ///
    /// Defaults to automatically determining
    hard_link_from_cache: bool,
}

impl Service for PathProvider {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<ServiceError>> {
        let options = services.get_service::<AppOptions>().await?;
        let mounts = services.get_service::<MountProvider>().await.ok();
        let paths = Self::new(options, mounts);
        paths.create_dirs().change_context(ServiceError::Create)?;
        Ok(paths)
    }
}

impl PathProvider {
    fn new(options: Arc<AppOptions>, mounts: Option<Arc<MountProvider>>) -> Self {
        let data_dir = options.data_dir.clone().unwrap_or_else(|| {
            data_dir()
                .expect("all platforms should have a data_dir")
                .join(APP_NAME)
        });
        let cache_dir = options.cache_dir.clone().unwrap_or_else(|| {
            cache_dir()
                .expect("all platforms should have a cache_dir")
                .join(APP_NAME)
        });
        let hard_link_from_cache = options
            .hard_link_from_cache
            .unwrap_or_else(|| default_for_hard_link_from_cache(mounts, &cache_dir, &data_dir));
        Self {
            data_dir,
            cache_dir,
            hard_link_from_cache,
        }
    }

    pub(crate) fn get_hard_link_from_cache(&self) -> bool {
        self.hard_link_from_cache
    }

    /// Directory for caching HTTP client responses.
    ///
    /// Default: `$HOME/.cache/alnwick/http` (or equivalent)
    #[must_use]
    pub fn get_http_dir(&self) -> PathBuf {
        self.cache_dir.join(HTTP_DIR)
    }

    /// Sqlite database for storing podcast metadata.
    ///
    /// Default: `$HOME/.local/share/alnwick/metadata.db` (or equivalent)
    #[must_use]
    pub fn get_metadata_db_path(&self) -> PathBuf {
        self.data_dir.join(METADATA_DB)
    }

    /// Directory for storing podcast episodes and feeds.
    ///
    /// Default: `$HOME/.local/share/alnwick/podcasts`
    #[must_use]
    pub fn get_podcasts_dir(&self) -> PathBuf {
        self.data_dir.join(PODCASTS_DIR)
    }

    /// Path for the RSS feed file.
    ///
    /// Examples:
    /// - `$HOME/.local/share/alnwick/podcasts/irl/feed.rss`
    /// - `$HOME/.local/share/alnwick/podcasts/irl/S00/feed.rss`
    /// - `$HOME/.local/share/alnwick/podcasts/irl/S00/1970/feed.rss`
    #[must_use]
    pub fn get_rss_path(
        &self,
        podcast_slug: &Slug,
        season: Option<u32>,
        year: Option<i32>,
    ) -> PathBuf {
        let path = self.get_podcasts_dir().join(podcast_slug.as_str());
        if season.is_none() && year.is_none() {
            return path.join(RSS_FILE_NAME);
        }
        let season = format!("S{:02}", season.unwrap_or(0));
        let year = year.map(|s| s.to_string()).unwrap_or_default();
        path.join(season).join(year).join(RSS_FILE_NAME)
    }

    /// Absolute path to where the cover image is stored.
    ///
    /// Example: `$HOME/.local/share/alnwick/podcasts/irl/cover.jpg`
    #[must_use]
    pub fn get_cover_path(&self, podcast_slug: &Slug) -> PathBuf {
        self.get_podcasts_dir()
            .join(podcast_slug.as_str())
            .join(COVER_FILE_NAME)
    }

    /// Absolute path to where the banner image is stored.
    ///
    /// Example: `$HOME/.local/share/alnwick/podcasts/irl/banner.jpg`
    #[must_use]
    pub fn get_banner_path(&self, podcast_slug: &Slug) -> PathBuf {
        self.get_podcasts_dir()
            .join(podcast_slug.as_str())
            .join(BANNER_FILE_NAME)
    }

    /// Create all the cache and data directories.
    pub fn create_dirs(&self) -> Result<(), Report<PathProviderError>> {
        let dirs = vec![
            ("Cache directory", self.cache_dir.clone()),
            ("Data directory", self.data_dir.clone()),
            ("HTTP cache directory", self.get_http_dir()),
            ("Podcasts directory", self.get_podcasts_dir()),
        ];
        for (name, dir) in dirs {
            if !dir.exists() {
                create_dir(&dir)
                    .change_context(PathProviderError::CreateDirectory(name.to_owned()))
                    .attach_path(dir)?;
            }
        }
        Ok(())
    }
}

#[cfg(target_os = "linux")]
fn default_for_hard_link_from_cache(
    mounts: Option<Arc<MountProvider>>,
    cache_dir: &Path,
    data_dir: &Path,
) -> bool {
    let mounts = mounts.expect("MountProvider should be available on linux platforms");
    let cache_mount = mounts
        .get_mount_id(cache_dir)
        .expect("should be able to get mount id of cache");
    let data_mount = mounts
        .get_mount_id(data_dir)
        .expect("should be able to get mount id of data");
    let is_match = cache_mount == data_mount;
    trace!(is_match, cache_mount = %cache_mount, data_mount = %data_mount, "Determining if cache and data are on the same mount");
    is_match
}

#[cfg(not(target_os = "linux"))]
fn default_for_hard_link_from_cache(
    mounts: Option<Arc<MountProvider>>,
    cache_dir: &Path,
    data_dir: &Path,
) -> bool {
    false
}

#[derive(Clone, Debug, Error)]
pub enum PathProviderError {
    #[error("Unable to create {0} directory")]
    CreateDirectory(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_rss_path() {
        // Arrange
        let paths = PathProvider {
            data_dir: PathBuf::default(),
            cache_dir: PathBuf::default(),
            hard_link_from_cache: false,
        };
        let data_dir = paths.data_dir.clone();
        let slug = Slug::from_str("abc").expect("should be valid slug");

        // Act
        // Assert
        assert_eq!(
            paths.get_rss_path(&slug, None, None),
            data_dir.join("podcasts/abc/feed.rss")
        );
        assert_eq!(
            paths.get_rss_path(&slug, Some(1), None),
            data_dir.join("podcasts/abc/S01/feed.rss")
        );
        assert_eq!(
            paths.get_rss_path(&slug, Some(1), Some(1234)),
            data_dir.join("podcasts/abc/S01/1234/feed.rss")
        );
        assert_eq!(
            paths.get_rss_path(&slug, None, Some(1234)),
            data_dir.join("podcasts/abc/S00/1234/feed.rss")
        );
    }

    #[tokio::test]
    #[allow(clippy::bool_assert_comparison)]
    async fn _get_hard_link_from_cache() {
        // Arrange
        let _logger = init_test_logger();
        let services = ServiceProvider::new();
        let paths = services
            .get_service::<PathProvider>()
            .await
            .expect("should be able to get path provider");

        // Act
        let result = paths.get_hard_link_from_cache();
        // Assert
        #[cfg(unix)]
        assert_eq!(result, true);
        #[cfg(not(unix))]
        assert_eq!(result, false);
    }
}
