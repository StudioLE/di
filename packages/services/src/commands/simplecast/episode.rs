use super::*;
use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[allow(clippy::struct_field_names)]
pub struct SimplecastEpisode {
    pub long_description: String,
    pub audio_status: String,
    pub image_url: Option<Url>,
    #[serde(rename = "type")]
    pub episode_type: String,
    pub token: String,
    pub description: String,
    pub slug: String,
    pub number: Option<usize>,
    pub audio_file: SimplecastAudioFile,
    pub audio_content_type: String,
    pub duration: Option<u64>,
    pub season: SimplecastSeason,
    pub title: String,
    pub episode_url: String,
    pub audio_file_size: u64,
    pub published_at: DateTime<FixedOffset>,
    pub href: Url,
    pub audio_file_path: String,
    pub enclosure_url: Url,
    pub authors: SimplecastAuthors,
    pub id: String,
    pub is_explicit: bool,
    pub podcast: SimplecastEpisodePodcast,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplecastAudioFile {
    pub url: String,
    pub size: u64,
    pub path_tc: String,
    pub path: String,
    pub name: String,
    pub href: Url,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplecastSeason {
    pub href: Url,
    pub number: usize,
    pub next_episode_number: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplecastEpisodePodcast {
    pub href: Url,
    pub title: String,
    pub image_url: Option<Url>,
    pub id: String,
    pub episodes: SimplecastCount,
    pub created_at: NaiveDateTime,
}

impl Display for SimplecastEpisode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let message = serde_yaml::to_string(self).unwrap_or_else(|_| format!("{self:?}"));
        write!(f, "{message}")
    }
}

impl From<SimplecastEpisode> for EpisodeInfo {
    fn from(episode: SimplecastEpisode) -> Self {
        EpisodeInfo {
            id: EpisodeInfo::determine_uuid(&episode.id),
            source_id: episode.id,
            title: episode.title,
            description: Some(episode.description),
            image: episode.image_url,
            source_url: episode.enclosure_url,
            kind: (&episode.episode_type).try_into().ok(),
            season: Some(episode.season.number),
            episode: episode.number,
            source_file_size: episode.audio_file_size,
            source_content_type: episode.audio_content_type,
            published_at: episode.published_at,
            source_duration: episode.duration,
            explicit: Some(episode.is_explicit),
            itunes_title: None,
        }
    }
}
