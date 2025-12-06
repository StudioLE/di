use crate::prelude::*;
use sea_orm::entity::prelude::*;

/// Episode type
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Display, PartialEq, Serialize, DeriveValueType,
)]
#[sea_orm(value_type = "String")]
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

impl FromStr for EpisodeKind {
    type Err = EpisodeKindError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let lower = value.to_lowercase();
        match lower.as_str() {
            "full" => Ok(EpisodeKind::Full),
            "trailer" => Ok(EpisodeKind::Trailer),
            "bonus" => Ok(EpisodeKind::Bonus),
            _ => Err(EpisodeKindError(value.to_owned())),
        }
    }
}

#[derive(Debug, Error)]
#[error("Unable to parse episode type: {0}")]
pub struct EpisodeKindError(String);
