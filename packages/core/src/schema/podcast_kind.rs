use crate::prelude::*;

/// Episodic or Serial
#[derive(Clone, Debug, Default, Deserialize, Display, PartialEq, Serialize)]
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

impl TryFrom<&String> for PodcastKind {
    type Error = Report<PodcastKindError>;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let lower = value.to_lowercase();
        match lower.as_str() {
            "episodic" => Ok(PodcastKind::Episodic),
            "serial" => Ok(PodcastKind::Serial),
            _ => Err(Report::new(PodcastKindError)
                .attach(format!("Invalid type: {value}"))),
        }
    }
}

#[derive(Debug, Error)]
#[error("Unable to parse podcast type")]
pub struct PodcastKindError;