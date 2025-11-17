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
    pub image_url: Option<Url>,
    pub published_at: DateTime<FixedOffset>,
    pub created_at: NaiveDateTime,
    pub is_explicit: bool,
    pub feed_url: Option<Url>,
    pub external_feed_url: Option<Url>,
}

impl From<SimplecastPodcast> for PodcastInfo {
    fn from(podcast: SimplecastPodcast) -> Self {
        PodcastInfo {
            id: podcast.id,
            title: podcast.title,
            description: podcast.description,
            image: podcast.image_url.map(|url| url.to_string()),
            language: Some(podcast.language),
            categories: Vec::new(),
            explicit: podcast.is_explicit,
            author: podcast.authors.collection.first().map(|a| a.name.clone()),
            link: Some(podcast.site.external_website.to_string()),
            kind: (&podcast.podcast_type).try_into().ok(),
            copyright: podcast.copyright,
            new_feed_url: None,
            generator: None,
        }
    }
}
