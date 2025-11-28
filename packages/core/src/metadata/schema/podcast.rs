use crate::prelude::*;
use sea_orm::entity::prelude::*;

/// Information about a podcast
///
/// - <https://help.apple.com/itc/podcasts_connect/#/itcb54353390>
/// - <https://github.com/Podcastindex-org/podcast-namespace>
pub type PodcastInfo = Model;

/// `SeaORM` Entity for [`PodcastInfo`]
#[allow(clippy::struct_field_names)]
#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel, Deserialize, PartialEq, Serialize)]
#[sea_orm(table_name = "podcasts")]
pub struct Model {
    // Database
    /// Primary key
    ///
    /// This is auto-incremented by the database
    #[sea_orm(primary_key)]
    pub primary_key: PodcastKey,

    /// Episodes related to this podcast
    #[sea_orm(has_many)]
    pub episodes: HasMany<episode::Entity>,

    // User
    /// User defined slug
    #[sea_orm(unique)]
    pub slug: Slug,

    // Required
    /// Title
    pub title: String,
    /// HTML formatted description
    pub description: String,
    /// URL of JPEG or PNG artwork
    /// - Min: 1400 x 1400 px
    /// - Max: 3000 x 3000 px
    pub image: Option<UrlWrapper>,
    /// ISO 639-2 code for language
    ///
    /// <https://www.loc.gov/standards/iso639-2/php/code_list.php>
    pub language: Option<String>,
    /// Categories
    ///
    /// <https://podcasters.apple.com/support/1691-apple-podcasts-categories>
    pub categories: PodcastCategories,
    /// Parental advisory information
    pub explicit: bool,

    // Recommended
    /// Group responsible for creating the show
    pub author: Option<String>,
    /// Website associated with the podcast
    pub link: Option<UrlWrapper>,

    // Situational
    /// Episodic or Serial
    pub kind: Option<PodcastKind>,
    /// Copyright details
    pub copyright: Option<String>,
    /// New podcast RSS Feed URL
    ///
    /// If you change the URL of your podcast feed, you should use this tag in your new feed
    pub new_feed_url: Option<UrlWrapper>,
    /// Program or hosting provider used to create the RSS feed
    pub generator: Option<String>,
}

impl PodcastInfo {
    #[must_use]
    pub fn example() -> Self {
        Self {
            slug: Slug::from_str("test").expect("should be able to parse slug"),
            primary_key: u32::default(),
            title: "Podcast Title".to_owned(),
            description: "Sed ac volutpat tortor. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Suspendisse placerat leo augue, id elementum orci venenatis eu.".to_owned(),
            image: None,
            language: Some("en-us".to_owned()),
            categories: PodcastCategories::default(),
            explicit: false,
            author: None,
            link: Some(UrlWrapper::from_str("https://example.com/").expect("should be able to parse URL")),
            kind: Some(PodcastKind::default()),
            copyright: None,
            new_feed_url: None,
            generator: None,
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
