use crate::prelude::*;

/// Information about a podcast
///
/// - <https://help.apple.com/itc/podcasts_connect/#/itcb54353390>
/// - <https://github.com/Podcastindex-org/podcast-namespace>
#[allow(clippy::struct_field_names)]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PodcastInfo {
    // Required
    /// App specific id
    pub id: String,
    /// Title
    pub title: String,
    /// HTML formatted description
    pub description: String,
    /// URL of JPEG or PNG artwork
    /// - Min: 1400 x 1400 px
    /// - Max: 3000 x 3000 px
    pub image: Option<String>,
    /// ISO 639-2 code for language
    ///
    /// <https://www.loc.gov/standards/iso639-2/php/code_list.php>
    pub language: Option<String>,
    /// Categories
    ///
    /// <https://podcasters.apple.com/support/1691-apple-podcasts-categories>
    pub categories: Vec<PodcastCategory>,
    /// Parental advisory information
    pub explicit: bool,

    // Recommended
    /// Group responsible for creating the show
    pub author: Option<String>,
    /// Website associated with the podcast
    pub link: Option<String>,

    // Situational
    /// Episodic or Serial
    pub kind: Option<PodcastKind>,
    /// Copyright details
    pub copyright: Option<String>,
    /// New podcast RSS Feed URL
    ///
    /// If you change the URL of your podcast feed, you should use this tag in your new feed
    pub new_feed_url: Option<String>,
    /// Program or hosting provider used to create the RSS feed
    pub generator: Option<String>,
}

impl PodcastInfo {
    #[must_use]
    pub fn get_image_url(&self) -> Option<Url> {
        self.image.clone().and_then(|url| {
            Url::parse(&url)
                .map_err(|error| {
                    warn!(podcast = self.id, %url, %error, "Failed to parse podcast image URL");
                })
                .ok()
        })
    }

    #[must_use]
    pub fn example() -> Self {
        Self {
            id: "test".to_owned(),
            title: "Podcast Title".to_owned(),
            description: "Sed ac volutpat tortor. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Suspendisse placerat leo augue, id elementum orci venenatis eu.".to_owned(),
            image: None,
            language: Some("en-us".to_owned()),
            categories: Vec::new(),
            explicit: false,
            author: None,
            link: Some("https://example.com/".to_owned()),
            kind: Some(PodcastKind::default()),
            copyright: None,
            new_feed_url: None,
            generator: None,
        }
    }
}
