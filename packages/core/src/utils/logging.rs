use crate::prelude::*;
use std::io::stderr;
use std::time::Instant;
use tracing::Level;
use tracing::dispatcher::SetGlobalDefaultError;
use tracing::level_filters::LevelFilter;
use tracing::subscriber::set_global_default;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Layer, Registry};

pub const DEFAULT_LOG_LEVEL: Level = Level::DEBUG;

pub fn init_logger() -> Result<(), SetGlobalDefaultError> {
    let targets = get_targets().with_default(LevelFilter::from_level(DEFAULT_LOG_LEVEL));
    let layer = layer()
        .compact()
        .with_writer(stderr)
        .with_target(false)
        .with_filter(targets);
    let registry = Registry::default().with(layer);
    set_global_default(registry)
}

pub fn init_elapsed_logger(level: Option<Level>) -> Result<(), SetGlobalDefaultError> {
    let level = level.unwrap_or(DEFAULT_LOG_LEVEL);
    let targets = get_targets().with_default(LevelFilter::from_level(level));
    let layer = layer()
        .compact()
        .with_writer(stderr)
        .with_target(false)
        // .with_timer(uptime())
        .with_timer(ElapsedTime::default())
        .with_filter(targets);
    let registry = Registry::default().with(layer);
    set_global_default(registry)
}

#[must_use]
pub fn get_targets() -> Targets {
    Targets::new()
        .with_target("cookie", LevelFilter::INFO)
        .with_target("html5ever", LevelFilter::INFO)
        .with_target("hyper_uti", LevelFilter::INFO)
        .with_target("lofty", LevelFilter::INFO)
        .with_target("reqwest", LevelFilter::INFO)
        .with_target("selectors", LevelFilter::INFO)
        .with_target("sqlx", LevelFilter::WARN)
}

struct ElapsedTime {
    start: Instant,
}

impl Default for ElapsedTime {
    fn default() -> Self {
        ElapsedTime {
            start: Instant::now(),
        }
    }
}

impl FormatTime for ElapsedTime {
    fn format_time(&self, w: &mut Writer<'_>) -> FmtResult {
        let elapsed = self.start.elapsed();
        write!(w, "{:.3}", elapsed.as_secs_f64())
    }
}
