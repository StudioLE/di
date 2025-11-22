use crate::prelude::*;
use sea_orm::entity::prelude::*;

/// - <https://www.sea-ql.org/SeaORM/docs/generate-entity/newtype/#wrapping-vect-backend-generic>
#[derive(Clone, Debug, Default, Deserialize, FromJsonQueryResult, PartialEq, Serialize)]
pub struct PodcastCategories(pub Vec<PodcastCategory>);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PodcastCategory {
    pub category: String,
    pub sub_category: Option<String>,
}
