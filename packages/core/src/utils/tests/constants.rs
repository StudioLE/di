use crate::prelude::*;
use sea_orm::{DatabaseBackend, Statement};
use sqlformat::{FormatOptions, QueryParams, format};

pub const PODCAST_KEY: u32 = 1;
pub const PODCAST_SLUG: &str = "irl";
pub const EPISODE_KEY: u32 = 1;
pub const EPISODE_YEAR: i32 = 2018;
pub const EPISODE_SEASON: u32 = 4;
pub const DB_BACKEND: DatabaseBackend = DatabaseBackend::Sqlite;

#[must_use]
pub(crate) fn example_slug() -> Slug {
    Slug::from_str(PODCAST_SLUG).expect("should be valid slug")
}

#[must_use]
pub(crate) fn example_rss_url() -> Url {
    Url::parse("https://feeds.simplecast.com/lP7owBq8").expect("URL should parse")
}

#[must_use]
pub(crate) fn example_simplecast_url() -> Url {
    Url::parse("https://irlpodcast.org").expect("URL should parse")
}

#[must_use]
pub(crate) fn format_sql(statement: &Statement) -> String {
    format(
        &statement.to_string(),
        &QueryParams::None,
        &FormatOptions::default(),
    )
}
