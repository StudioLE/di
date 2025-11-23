use sea_orm::{DatabaseBackend, Statement};
use sqlformat::{format, FormatOptions, QueryParams};

pub const PODCAST_KEY: u32 = 1;
pub const PODCAST_SLUG: &str = "irl";
pub const EPISODE_KEY: u32 = 1;
pub const DB_BACKEND: DatabaseBackend = DatabaseBackend::Sqlite;

#[must_use] 
pub(crate) fn format_sql(statement: &Statement) -> String {
    format(
        &statement.to_string(),
        &QueryParams::None,
        &FormatOptions::default(),
    )
}
