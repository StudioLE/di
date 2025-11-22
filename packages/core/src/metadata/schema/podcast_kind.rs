use crate::prelude::*;
use sea_orm::entity::prelude::*;

/// Episodic or Serial
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Display, PartialEq, Serialize, DeriveValueType,
)]
#[sea_orm(value_type = "String")]
pub enum PodcastKind {
    /// Specify episodic when episodes are intended to be consumed without any specific order.
    /// Apple Podcasts will present newest episodes first and display the publish date (required)
    /// of each episode. If organized into seasons, the newest season will be presented first -
    /// otherwise, episodes will be grouped by year published, newest first.
    #[default]
    Episodic,
    /// Specify serial when episodes are intended to be consumed in sequential order. Apple
    /// Podcasts will present the oldest episodes first and display the episode numbers (required)
    /// of each episode. If organized into seasons, the newest season will be presented first and
    /// <itunes:episode> numbers must be given for each episode.
    Serial,
}

impl FromStr for PodcastKind {
    type Err = Report<PodcastKindError>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let lower = value.to_lowercase();
        match lower.as_str() {
            "episodic" => Ok(PodcastKind::Episodic),
            "serial" => Ok(PodcastKind::Serial),
            _ => Err(Report::new(PodcastKindError).attach(format!("Invalid type: {value}"))),
        }
    }
}

#[derive(Debug, Error)]
#[error("Unable to parse podcast type")]
pub struct PodcastKindError;
