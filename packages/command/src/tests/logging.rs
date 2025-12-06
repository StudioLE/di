use crate::prelude::*;
use std::io::stderr;
use std::sync::OnceLock;
use std::time::Instant;
use tracing::level_filters::LevelFilter;
use tracing::subscriber::set_global_default;
use tracing::Level;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Layer, Registry};

static INIT: OnceLock<()> = OnceLock::new();

const DEFAULT_LOG_LEVEL: Level = Level::TRACE;

pub fn init_elapsed_logger() {
    INIT.get_or_init(|| {
        let targets = get_targets().with_default(LevelFilter::from_level(DEFAULT_LOG_LEVEL));
        let layer = layer()
            .compact()
            .with_writer(stderr)
            .with_target(false)
            .with_timer(ElapsedTime::default())
            .with_filter(targets);
        let registry = Registry::default().with(layer);
        set_global_default(registry).expect("should be able to set global default");
    });
}

#[must_use]
pub fn get_targets() -> Targets {
    Targets::new()
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
