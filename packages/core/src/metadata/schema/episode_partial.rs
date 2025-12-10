use crate::prelude::*;
use sea_orm::*;

/// A partial of [`episode::Model`]
///
/// Used by:
/// - [`MetadataRepository::get_podcast`]
/// - [`MetadataRepository::get_episode`]
#[derive(Clone, Debug, FromQueryResult, Deserialize, PartialEq, Serialize)]
pub struct EpisodePartial {
    /// Primary key
    ///
    /// This is auto-incremented by the database
    pub primary_key: u32,
    /// Title
    pub title: String,
    /// Date and time episode was released
    pub published_at: DateTime<FixedOffset>,
    /// HTML formatted description
    ///
    /// This will always be `None` for [`MetadataRepository::get_podcast`]
    pub description: Option<String>,
    /// Duration in seconds
    pub source_duration: Option<u32>,
    /// URL of JPEG or PNG artwork
    /// - Min: 1400 x 1400 px
    /// - Max: 3000 x 3000 px
    pub image: Option<UrlWrapper>,
    /// Episode number
    pub episode: Option<u32>,
    /// Season number
    pub season: Option<u32>,
    /// Episode type
    pub kind: Option<EpisodeKind>,
    /// Relative file path to the downloaded audio file.
    ///
    /// Value will be `None` until the file is downloaded with [`DownloadContext`].
    pub file_sub_path: Option<PathWrapper>,
    /// Relative file path to the downloaded image file.
    ///
    /// Value will be `None` until the file is downloaded with [`DownloadContext`].
    pub image_sub_path: Option<PathWrapper>,
}
