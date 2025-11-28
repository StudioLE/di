use crate::prelude::*;
use sea_orm::*;

/// A partial of [`podcast::Model`]
///
/// Used by:
/// - [`MetadataRepository::get_podcasts`]
/// - [`MetadataRepository::get_podcast`]
#[derive(Clone, Debug, FromQueryResult, Deserialize, PartialEq, Serialize)]
pub struct PodcastPartial {
    /// Primary key
    ///
    /// This is auto-incremented by the database
    pub primary_key: u32,
    /// User defined slug
    pub slug: Slug,
    /// Title
    pub title: String,
    /// URL of JPEG or PNG artwork
    /// - Min: 1400 x 1400 px
    /// - Max: 3000 x 3000 px
    pub image: Option<UrlWrapper>,
    /// Episode count
    pub episodes_count: u32,
}
