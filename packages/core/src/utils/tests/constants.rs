use crate::prelude::*;
use sea_orm::{DatabaseBackend, Statement};
use sqlformat::{FormatOptions, QueryParams, format};

pub const PODCAST_KEY: PodcastKey = 1;
const SS_PODCAST_SLUG: &str = "irl";
const PODCAST_SLUG: &str = "test-0";
pub const EPISODE_KEY: EpisodeKey = 2;
pub const DB_BACKEND: DatabaseBackend = DatabaseBackend::Sqlite;

#[must_use]
#[deprecated]
pub(crate) fn example_slug() -> Slug {
    Slug::from_str(SS_PODCAST_SLUG).expect("should be valid slug")
}

#[must_use]
pub(crate) fn podcast_slug() -> Slug {
    Slug::from_str(PODCAST_SLUG).expect("should be valid slug")
}

#[must_use]
pub(crate) fn example_rss_url() -> UrlWrapper {
    UrlWrapper::from_str("https://feeds.simplecast.com/lP7owBq8").expect("URL should parse")
}

#[must_use]
pub(crate) fn example_simplecast_url() -> UrlWrapper {
    UrlWrapper::from_str("https://irlpodcast.org").expect("URL should parse")
}

#[must_use]
pub(crate) fn format_sql(statement: &Statement) -> String {
    format(
        &statement.to_string(),
        &QueryParams::None,
        &FormatOptions::default(),
    )
}
