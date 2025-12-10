use std::io::stderr;
use tracing::Level;
use tracing::dispatcher::SetGlobalDefaultError;
use tracing::level_filters::LevelFilter;
use tracing::subscriber::set_global_default;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::layer;
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

#[must_use]
pub fn get_targets() -> Targets {
    Targets::new()
        .with_target("cookie", LevelFilter::INFO)
        .with_target("html5ever", LevelFilter::INFO)
        .with_target("hyper_uti", LevelFilter::INFO)
        .with_target("lofty", LevelFilter::INFO)
        .with_target("reqwest", LevelFilter::INFO)
        .with_target("selectors", LevelFilter::INFO)
        .with_target("sqlx", LevelFilter::TRACE)
}
