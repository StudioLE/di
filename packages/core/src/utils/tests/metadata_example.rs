use crate::prelude::*;
use base64::prelude::*;
use chrono::{NaiveDate, NaiveTime, TimeZone};

pub struct MetadataRepositoryExample;

impl MetadataRepositoryExample {
    pub const YEAR_COUNT: u32 = 3;
    pub const SEASONS_PER_YEAR: u32 = 2;
    pub const START_YEAR: u32 = 2000;
    pub const EPISODES_PER_SEASON: u32 = 3;

    #[allow(clippy::as_conversions)]
    pub const PODCAST_COUNT: usize = 3;
    #[allow(clippy::as_conversions)]
    pub const EPISODE_COUNT: usize = Self::PODCAST_COUNT * Self::EPISODES_PER_SEASON as usize;
    pub const PODCAST_SLUG: &'static str = "test-0";
    const EPISODE_FILE_URL: &'static str = "aHR0cHM6Ly9maWxlcy5mcmVlbXVzaWNhcmNoaXZlLm9yZy9zdG9yYWdlLWZyZWVtdXNpY2FyY2hpdmUtb3JnL3RyYWNrcy9nR1J5M1JmYm1EWE5vOEw1SlBPc0I3ZFBoTXhnbEJKaEw4M2owVHp5Lm1wMw==";
    const EPISODE_IMAGE_URL: &'static str =
        "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png";

    pub async fn create() -> MetadataRepository {
        let dir = TempDirectory::default()
            .create()
            .expect("should be able to create temp directory");
        Self::create_in_directory(dir).await
    }

    pub async fn create_in_directory(dir: PathBuf) -> MetadataRepository {
        let path = dir.join(METADATA_DB);
        let metadata = MetadataRepository::new(path)
            .await
            .expect("should be able to create metadata repository");
        metadata
            .migrate()
            .await
            .expect("should be able to migrate metadata database");
        for feed in Self::example_feeds() {
            metadata
                .create_feed(feed)
                .await
                .expect("should be able to save feed");
        }
        metadata
    }

    #[must_use]
    pub fn podcast_slug() -> Slug {
        Slug::from_str(Self::PODCAST_SLUG).expect("should be valid slug")
    }

    #[must_use]
    pub fn get_image_url() -> UrlWrapper {
        UrlWrapper::from_str(Self::EPISODE_IMAGE_URL).expect("should be valid URL")
    }

    fn get_episode_file_url() -> UrlWrapper {
        let bytes = BASE64_STANDARD
            .decode(Self::EPISODE_FILE_URL)
            .expect("should be valid base64");
        let url = String::from_utf8(bytes).expect("should be valid UTF-8");
        UrlWrapper::from_str(&url).expect("should be valid URL")
    }

    #[must_use]
    pub fn example_feeds() -> Vec<PodcastFeed> {
        let mut feeds = Vec::new();
        let source_url = Self::get_episode_file_url();
        let image_url = Self::get_image_url();
        for podcast_index in 0..Self::PODCAST_COUNT {
            let mut episodes = Vec::new();
            let slug = Slug::from_str(&format!("test-{podcast_index}")).expect("should be valid");
            let podcast = PodcastInfo {
                title: format!("Podcast {podcast_index}"),
                slug: slug.clone(),
                ..PodcastInfo::example()
            };
            for year_i in 0..Self::YEAR_COUNT {
                let year = Self::START_YEAR + year_i;
                for season_i in 1..=Self::SEASONS_PER_YEAR {
                    let season = year_i * Self::SEASONS_PER_YEAR + season_i;
                    for episode in 1..=Self::EPISODES_PER_SEASON {
                        let ordinal = season_i * 100 + episode * 7;
                        episodes.push(EpisodeInfo {
                            title: format!("S{season:02}E{episode:02} of {slug}"),
                            published_at: date(year, ordinal),
                            season: Some(season),
                            episode: Some(episode),
                            source_url: source_url.clone(),
                            image: Some(image_url.clone()),
                            ..EpisodeInfo::example()
                        });
                    }
                }
            }
            feeds.push(PodcastFeed { podcast, episodes });
        }
        feeds
    }
}

#[allow(clippy::as_conversions, clippy::cast_possible_wrap)]
fn date(year: u32, ordinal: u32) -> DateTime<FixedOffset> {
    let date = NaiveDate::from_yo_opt(year as i32, ordinal).expect("should be a valid date");
    let datetime = NaiveDateTime::new(
        date,
        NaiveTime::from_hms_opt(0, 0, 0).expect("should be a valid time"),
    );
    let offset = FixedOffset::east_opt(0).expect("should be a valid offset");

    offset.from_utc_datetime(&datetime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _example_feeds() {
        // Arrange
        // Act
        let feeds = MetadataRepositoryExample::example_feeds();

        // Assert
        assert_eq!(
            feeds.len(),
            MetadataRepositoryExample::PODCAST_COUNT,
            "podcast count"
        );
        assert_yaml_snapshot!(feeds);
    }

    #[test]
    fn _get_episode_file_url() {
        // Arrange
        // Act
        // Assert
        let _url = MetadataRepositoryExample::get_episode_file_url();
    }
}
