use crate::prelude::*;
use dirs::{cache_dir, data_dir};
use std::fs::create_dir;

const HTTP_DIR: &str = "http";
const PODCASTS_DIR: &str = "podcasts";
const TORRENT_DIR: &str = "torrent";
const TORRENT_CONTENT_DIR: &str = "content";
const TORRENT_FILES_DIR: &str = "files";
const RSS_FILE_NAME: &str = "feed.rss";
const METADATA_DB: &str = "metadata.db";
const BANNER_FILE_NAME: &str = "banner.jpg";
const COVER_FILE_NAME: &str = "cover.jpg";

/// Service for providing file paths and URL.
#[derive(Default)]
pub struct PathProvider {
    options: AppOptions,
}

impl PathProvider {
    /// Create a new `PathProvider`.
    #[must_use]
    pub fn new(options: AppOptions) -> Self {
        Self { options }
    }

    /// Directory for app data.
    ///
    /// Default: `$HOME/.local/share/alnwick` (or equivalent)
    fn get_data_dir(&self) -> PathBuf {
        self.options.data_dir.clone().unwrap_or_else(|| {
            data_dir()
                .expect("all platforms should have a data_dir")
                .join(APP_NAME)
        })
    }

    /// Directory for app cache.
    ///
    /// Default: `$HOME/.cache/alnwick` (or equivalent)
    fn get_cache_dir(&self) -> PathBuf {
        self.options.cache_dir.clone().unwrap_or_else(|| {
            cache_dir()
                .expect("all platforms should have a cache_dir")
                .join(APP_NAME)
        })
    }

    /// Directory for caching HTTP client responses.
    ///
    /// Default: `$HOME/.cache/alnwick/http` (or equivalent)
    #[must_use]
    pub fn get_http_dir(&self) -> PathBuf {
        self.get_cache_dir().join(HTTP_DIR)
    }

    /// Sqlite database for storing podcast metadata.
    ///
    /// Default: `$HOME/.local/share/alnwick/metadata.db` (or equivalent)
    #[must_use]
    pub fn get_metadata_db_path(&self) -> PathBuf {
        self.get_data_dir().join(METADATA_DB)
    }

    /// Directory for storing podcast episodes and feeds.
    ///
    /// Default: `$HOME/.local/share/alnwick/podcasts`
    fn get_podcasts_dir(&self) -> PathBuf {
        self.get_data_dir().join(PODCASTS_DIR)
    }

    /// Directory for storing torrent content and files.
    ///
    /// Default: `$HOME/.local/share/alnwick/torrent`
    fn get_torrent_dir(&self) -> PathBuf {
        self.get_data_dir().join(TORRENT_DIR)
    }

    /// Directory for storing torrent content.
    ///
    /// Default: `$HOME/.local/share/alnwick/torrent/content`
    #[must_use]
    pub fn get_torrent_content_dir(&self) -> PathBuf {
        self.get_torrent_dir().join(TORRENT_CONTENT_DIR)
    }

    /// Directory for storing torrent files.
    ///
    /// Default: `$HOME/.local/share/alnwick/torrent/files`
    #[must_use]
    pub fn get_torrent_files_dir(&self) -> PathBuf {
        self.get_torrent_dir().join(TORRENT_FILES_DIR)
    }

    /// Absolute path to where the downloaded and processed episode audio file is stored.
    ///
    /// Example: `$HOME/.local/share/alnwick/podcasts/irl/S00/1970/1970-01-01 001 Hello World.mp3`
    #[must_use]
    pub fn get_audio_path(&self, podcast_slug: &Slug, episode: &EpisodeInfo) -> PathBuf {
        self.get_podcasts_dir()
            .join(get_sub_path_for_audio(podcast_slug, episode))
    }

    /// URL of the episode audio file.
    ///
    /// If the `server_base` option is not set this falls back to a `file://` URL.
    ///
    /// Examples:
    /// - `https://example.com/irl/S00/1970/1970-01-01 001 Hello World.mp3`
    /// - `file://$HOME/.local/share/alnwick/podcasts/irl/S00/1970/1970-01-01 001 Hello World.mp3`
    #[must_use]
    pub fn get_audio_url(&self, podcast_slug: &Slug, episode: &EpisodeInfo) -> Option<Url> {
        if let Some(base) = &self.options.server_base {
            let path = get_sub_path_for_audio(podcast_slug, episode);
            base.join(path.to_string_lossy().as_ref()).ok()
        } else {
            let path = self.get_audio_path(podcast_slug, episode);
            Url::from_file_path(path).ok()
        }
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
        let season = EpisodeInfo::format_season(season);
        let year = year.map(|s| s.to_string()).unwrap_or_default();
        path.join(season).join(year).join(RSS_FILE_NAME)
    }

    /// Sub path for audio files as torrent content.
    ///
    /// Examples:
    /// - `S00/1970/1970-01-01 001 Hello World.mp3`
    /// - `S00/1970-01-01 001 Hello World.mp3`
    /// - `1970/1970-01-01 001 Hello World.mp3`
    #[must_use]
    pub fn get_torrent_sub_path(
        season_dirs: bool,
        year_dirs: bool,
        episode: &EpisodeInfo,
    ) -> PathBuf {
        let mut path = PathBuf::new();
        if season_dirs {
            path.push(episode.get_formatted_season());
        }
        if year_dirs {
            path.push(episode.published_at.year().to_string());
        }
        path.join(episode.get_filename())
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
    pub fn create(&self) -> Result<(), Report<ServiceError>> {
        let cache_dir = self.get_cache_dir();
        let data_dir = self.get_data_dir();
        let dirs = vec![
            ("Cache directory", cache_dir),
            ("Data directory", data_dir),
            ("HTTP cache directory", self.get_http_dir()),
            ("Podcasts directory", self.get_podcasts_dir()),
            ("Torrent directory", self.get_torrent_dir()),
            ("Torrent content directory", self.get_torrent_content_dir()),
            ("Torrent files directory", self.get_torrent_files_dir()),
        ];
        for (name, dir) in dirs {
            if !dir.exists() {
                create_dir(&dir)
                    .change_context(ServiceError::CreateDirectory(name.to_owned()))
                    .attach_path(dir)?;
            }
        }
        Ok(())
    }
}

/// Sub path for an episodes's audio file.
///
/// Example: `irl/S00/1970/1970-01-01 001 Hello World.mp3`
fn get_sub_path_for_audio(podcast_slug: &Slug, episode: &EpisodeInfo) -> PathBuf {
    let season = episode.get_formatted_season();
    let year = episode.published_at.year().to_string();
    let filename = episode.get_filename();
    PathBuf::new()
        .join(podcast_slug.as_str())
        .join(season)
        .join(year)
        .join(filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_audio_path() {
        // Arrange
        let paths = PathProvider::default();
        let data_dir = paths.get_data_dir();
        let example = EpisodeInfo::example();
        let mut seasonless = EpisodeInfo::example();
        seasonless.season = None;
        let slug = Slug::from_str("abc").expect("should be valid slug");

        // Act
        // Assert
        assert_eq!(
            paths.get_audio_path(&slug, &example),
            data_dir.join("podcasts/abc/S02/1970/1970-01-01 003 Lorem ipsum dolor sit amet.mp3")
        );
        assert_eq!(
            paths.get_audio_path(&slug, &seasonless),
            data_dir.join("podcasts/abc/S00/1970/1970-01-01 003 Lorem ipsum dolor sit amet.mp3")
        );
    }

    #[test]
    fn get_audio_url_file() {
        // Arrange
        let paths = PathProvider::default();
        let data_dir = paths.get_data_dir();
        let expected =
            data_dir.join("podcasts/abc/S02/1970/1970-01-01 003 Lorem ipsum dolor sit amet.mp3");
        let expected = Url::from_file_path(expected).expect("should be valid");
        let slug = Slug::from_str("abc").expect("should be valid slug");

        // Act
        let result = paths
            .get_audio_url(&slug, &EpisodeInfo::example())
            .expect("should be valid");

        // Assert
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn get_audio_url_http() {
        // Arrange
        let options = AppOptions {
            server_base: Some(Url::parse("https://example.com").expect("should be valid")),
            ..AppOptions::default()
        };
        let paths = PathProvider::new(options);
        let slug = Slug::from_str("abc").expect("should be valid slug");

        // Act
        let result = paths
            .get_audio_url(&slug, &EpisodeInfo::example())
            .expect("should be valid");

        // Assert
        assert_eq!(result.to_string(), "https://example.com/abc/S02/1970/1970-01-01%20003%20Lorem%20ipsum%20dolor%20sit%20amet.mp3".to_owned());
    }

    #[test]
    fn get_rss_path() {
        // Arrange
        let paths = PathProvider::default();
        let data_dir = paths.get_data_dir();
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
}
