/// File size in bytes.
///
/// Note: `SQLx` does not support `u64` so `i64` is necessary even though values will never be negative.
pub type FileSize = i64;

pub type PodcastKey = u32;

pub type EpisodeKey = u32;

/// Duration in seconds.
pub type Duration = u32;

pub type EpisodeNumber = u32;

pub type SeasonNumber = u32;
