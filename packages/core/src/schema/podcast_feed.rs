use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PodcastFeed {
    pub podcast: PodcastInfo,
    pub episodes: Vec<EpisodeInfo>,
}

impl PodcastFeed {
    #[must_use]
    pub fn example() -> Self {
        Self {
            podcast: PodcastInfo::example(),
            episodes: vec![EpisodeInfo::example()],
        }
    }
}

impl From<(PodcastInfo, Vec<EpisodeInfo>)> for PodcastFeed {
    fn from((podcast, episodes): (PodcastInfo, Vec<EpisodeInfo>)) -> Self {
        Self { podcast, episodes }
    }
}
