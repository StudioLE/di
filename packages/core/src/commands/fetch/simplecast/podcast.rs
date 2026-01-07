use super::*;
use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplecastPodcast {
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    pub podcast_type: String,
    pub site: SimplecastSite,
    pub language: String,
    pub authors: SimplecastAuthors,
    pub copyright: Option<String>,
    pub image_url: Option<UrlWrapper>,
    pub published_at: DateTime<FixedOffset>,
    pub created_at: NaiveDateTime,
    pub is_explicit: bool,
    pub feed_url: Option<UrlWrapper>,
    pub external_feed_url: Option<UrlWrapper>,
}

impl From<SimplecastPodcast> for PodcastInfo {
    fn from(podcast: SimplecastPodcast) -> Self {
        PodcastInfo {
            primary_key: u32::default(),
            slug: Slug::from_str(&podcast.id).expect("should be valid slug"),
            feed_url: None,
            title: podcast.title,
            description: podcast.description,
            image: podcast.image_url,
            language: Some(podcast.language),
            categories: PodcastCategories::default(),
            explicit: podcast.is_explicit,
            author: podcast.authors.collection.first().map(|a| a.name.clone()),
            link: Some(podcast.site.external_website),
            kind: PodcastKind::from_str(&podcast.podcast_type).ok(),
            copyright: podcast.copyright,
            new_feed_url: None,
            generator: None,
        }
    }
}
