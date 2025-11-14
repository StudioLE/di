use crate::prelude::*;

/// Episode type
#[derive(Clone, Copy, Debug, Default, Deserialize, Display, PartialEq, Serialize)]
pub enum EpisodeKind {
    /// Complete content
    #[default]
    Full,
    /// Short promotional piece
    /// - Show trailer has no season or episode number
    /// - Season trailer has a season number and no episode number
    /// - Episode trailer has an episode number and optionally a season number
    Trailer,
    /// Extra content
    /// - Show bonus has no season or episode number
    /// - Season bonus has a season number and no episode number
    /// - Episode specific bonus has an episode number and optionally a season number
    Bonus,
}

impl TryFrom<&String> for EpisodeKind {
    type Error = Report<EpisodeKindError>;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let lower = value.to_lowercase();
        match lower.as_str() {
            "full" => Ok(EpisodeKind::Full),
            "trailer" => Ok(EpisodeKind::Trailer),
            "bonus" => Ok(EpisodeKind::Bonus),
            _ => Err(Report::new(EpisodeKindError)
                .attach(format!("Invalid type: {value}"))),
        }
    }
}

#[derive(Debug, Error)]
#[error("Unable to parse episode type")]
pub struct EpisodeKindError;
