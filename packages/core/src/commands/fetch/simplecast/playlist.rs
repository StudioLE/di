use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplecastPlaylist {
    pub href: UrlWrapper,
    #[allow(clippy::struct_field_names)]
    #[serde(rename = "type")]
    pub playlist_type: String,
    pub title: String,
    pub image_url: UrlWrapper,
    pub feed_url: UrlWrapper,
    pub episodes: SimplecastEpisodes,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplecastEpisodes {
    pub pages: SimplecastPages,
    pub collection: Vec<SimplecastPlaylistEpisode>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplecastPages {
    pub total: usize,
    pub previous: Option<SimplecastLink>,
    pub next: Option<SimplecastLink>,
    pub limit: usize,
    pub current: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplecastLink {
    pub href: UrlWrapper,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplecastPlaylistEpisode {
    #[allow(clippy::struct_field_names)]
    #[serde(rename = "type")]
    pub episode_type: String,
    pub title: String,
    pub season_number: Option<usize>,
    pub number: Option<usize>,
    pub image_url: Option<UrlWrapper>,
    pub id: String,
    pub enclosure_url: UrlWrapper,
    pub duration: usize,
}
