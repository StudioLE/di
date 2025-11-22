use crate::prelude::*;

#[derive(Args, Clone, Debug, Default, Serialize)]
pub struct FilterOptions {
    /// Only include episodes with the specified season
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<u32>,
    /// Exclude episodes before the specified season
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_season: Option<u32>,
    /// Exclude episodes after the specified season
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_season: Option<u32>,
    /// Only include episodes with the specified year
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    /// Exclude episodes before the specified year
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_year: Option<i32>,
    /// Exclude episodes after the specified year
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_year: Option<i32>,
}

impl Display for FilterOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let yaml = serde_yaml::to_string(self).expect("should be able to serialize");
        let output = yaml.trim_end().replace("\r\n", " ").replace('\n', " ");
        write!(f, "{output}")
    }
}

impl PodcastFeed {
    pub fn filter(&mut self, options: &FilterOptions) {
        let before = self.episodes.len();
        self.episodes.retain(|episode| {
            if let Some(year) = options.year
                && episode.published_at.year() != year
            {
                return false;
            }
            if let Some(year) = options.from_year
                && episode.published_at.year() < year
            {
                return false;
            }
            if let Some(year) = options.to_year
                && episode.published_at.year() > year
            {
                return false;
            }
            if let Some(season) = options.season {
                let Some(episode_season) = episode.season else {
                    return false;
                };
                if episode_season != season {
                    return false;
                }
            }
            if let Some(season) = options.from_season {
                let Some(episode_season) = episode.season else {
                    return false;
                };
                if episode_season < season {
                    return false;
                }
            }
            if let Some(season) = options.to_season {
                let Some(episode_season) = episode.season else {
                    return false;
                };
                if episode_season > season {
                    return false;
                }
            }
            true
        });
        let after = self.episodes.len();
        debug!("Filter includes {after} of {before} episodes");
        trace!("Options: {options}");
    }
}
