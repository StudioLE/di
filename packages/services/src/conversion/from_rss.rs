use crate::prelude::*;

pub struct PodcastFromRss;

impl PodcastFromRss {
    pub fn execute(
        mut channel: RssChannel,
        id: &str,
    ) -> Result<PodcastFeed, Report<PodcastFromRssError>> {
        let items = take(&mut channel.items);
        let podcast = podcast_from_rss(channel, id)?;
        let mut episodes = Vec::new();
        let mut report: Option<Report<[EpisodeFromRssError]>> = None;
        for item in items {
            let name = item
                .title
                .clone()
                .unwrap_or_else(|| item.guid.clone().unwrap_or_default().value);
            match episode_from_rss(item) {
                Ok(episode) => episodes.push(episode),
                Err(error) => {
                    let error = error.attach(format!("Episode: {name}",));
                    if let Some(report) = report.as_mut() {
                        report.push(error);
                    } else {
                        report = Some(error.expand());
                    }
                }
            }
        }
        if let Some(report) = report {
            return Err(report.change_context(PodcastFromRssError::ParseEpisodes));
        }
        let feed = PodcastFeed { podcast, episodes };
        Ok(feed)
    }
}

fn podcast_from_rss(
    channel: RssChannel,
    id: &str,
) -> Result<PodcastInfo, Report<PodcastFromRssError>> {
    let itunes = channel.itunes_ext.ok_or(PodcastFromRssError::NoItunes)?;
    let podcast = PodcastInfo {
        id: id.to_owned(),
        title: channel.title,
        description: channel.description,
        image: if let Some(url) = &itunes.image {
            Some(try_parse_url(url, PodcastFromRssError::ParseImage)?)
        } else {
            None
        },
        language: channel.language,
        categories: itunes
            .categories
            .into_iter()
            .map(|category| PodcastCategory {
                category: category.text,
                sub_category: category.subcategory.map(|sub_category| sub_category.text),
            })
            .collect(),
        explicit: false,
        author: itunes.author,
        link: Some(try_parse_url(
            &channel.link,
            PodcastFromRssError::ParseLink,
        )?),
        kind: if let Some(kind) = &itunes.r#type {
            Some(PodcastKind::try_from(kind).change_context(PodcastFromRssError::ParseKind)?)
        } else {
            None
        },
        copyright: channel.copyright,
        new_feed_url: itunes.new_feed_url,
        generator: channel.generator,
    };
    Ok(podcast)
}

fn episode_from_rss(item: RssItem) -> Result<EpisodeInfo, Report<EpisodeFromRssError>> {
    let itunes = item.itunes_ext.ok_or(EpisodeFromRssError::NoItunes)?;
    let enclosure = item.enclosure.ok_or(EpisodeFromRssError::NoEnclosure)?;
    let source_id = item.guid.ok_or(EpisodeFromRssError::NoGuid)?.value;
    let pub_date = &item.pub_date.ok_or(EpisodeFromRssError::NoPublishedAt)?;
    let published_at = DateTime::parse_from_rfc2822(pub_date)
        .change_context(EpisodeFromRssError::ParsePublishedAt)?;
    let episode = EpisodeInfo {
        title: item.title.ok_or(EpisodeFromRssError::NoTitle)?,
        source_url: try_parse_url(&enclosure.url, EpisodeFromRssError::ParseUrl)?,
        source_file_size: try_parse(&enclosure.length, EpisodeFromRssError::ParseFileSize)?,
        source_content_type: enclosure.mime_type,
        id: EpisodeInfo::determine_uuid(&source_id),
        source_id,
        published_at,
        description: item.description,
        source_duration: if let Some(duration) = itunes.duration {
            Some(try_parse_duration(&duration)?)
        } else {
            None
        },
        image: if let Some(url) = &itunes.image {
            Some(try_parse_url(url, EpisodeFromRssError::ParseImage)?)
        } else {
            None
        },
        explicit: parse_explicit(itunes.explicit),
        itunes_title: itunes.subtitle,
        episode: if let Some(episode) = itunes.episode {
            Some(try_parse(&episode, EpisodeFromRssError::ParseEpisode)?)
        } else {
            None
        },
        season: if let Some(season) = itunes.season {
            Some(try_parse(&season, EpisodeFromRssError::ParseSeason)?)
        } else {
            None
        },
        kind: if let Some(kind) = &itunes.episode_type {
            Some(EpisodeKind::try_from(kind).change_context(EpisodeFromRssError::ParseKind)?)
        } else {
            None
        },
    };
    Ok(episode)
}

#[allow(clippy::indexing_slicing)]
fn try_parse_duration(duration: &str) -> Result<u64, Report<EpisodeFromRssError>> {
    let parts: Vec<&str> = duration.split(':').collect();
    match parts.len() {
        1 => try_parse(parts[0], EpisodeFromRssError::ParseDuration),
        2 => {
            let minutes: u64 = try_parse(parts[0], EpisodeFromRssError::ParseDuration)?;
            let seconds: u64 = try_parse(parts[1], EpisodeFromRssError::ParseDuration)?;
            Ok(minutes * 60 + seconds)
        }
        3 => {
            let hours: u64 = try_parse(parts[0], EpisodeFromRssError::ParseDuration)?;
            let minutes: u64 = try_parse(parts[1], EpisodeFromRssError::ParseDuration)?;
            let seconds: u64 = try_parse(parts[2], EpisodeFromRssError::ParseDuration)?;
            Ok(hours * 60 * 60 + minutes * 60 + seconds)
        }
        count => {
            Err(Report::new(EpisodeFromRssError::ParseDuration).attach(format!("Parts: {count}")))
        }
    }
}

fn parse_explicit(option: Option<String>) -> Option<bool> {
    let lower = option?.to_lowercase();
    match lower.as_str() {
        "true" | "yes" | "explicit" => Some(true),
        "false" | "no" | "clean" => Some(false),
        _ => None,
    }
}

fn try_parse_url<E: Error + Send + Sync + 'static>(
    url: &String,
    error: E,
) -> Result<Url, Report<E>> {
    Url::parse(url)
        .change_context(error)
        .attach_with(|| format!("URL: {url}"))
}

fn try_parse<T: FromStr, E: Error + Send + Sync + 'static>(
    value: &str,
    error: E,
) -> Result<T, Report<E>> {
    match value.parse::<T>() {
        Ok(parsed) => Ok(parsed),
        Err(_) => Err(Report::new(error).attach(format!("Value: {value}"))),
    }
}

#[derive(Debug, Error)]
pub enum PodcastFromRssError {
    #[error("Podcast RSS channel is missing the itunes extension")]
    NoItunes,
    #[error("Unable to parse podcast image URL")]
    ParseImage,
    #[error("Unable to parse podcast link URL")]
    ParseLink,
    #[error("Unable to parse podcast type")]
    ParseKind,
    #[error("Unable to parse all episodes")]
    ParseEpisodes,
}

#[derive(Debug, Error)]
pub enum EpisodeFromRssError {
    #[error("RSS channel is missing the itunes extension")]
    NoItunes,
    #[error("Episode has no GUID")]
    NoGuid,
    #[error("Episode has no title")]
    NoTitle,
    #[error("Episode does not have an enclosure")]
    NoEnclosure,
    #[error("Unable to parse enclosure URL")]
    ParseUrl,
    #[error("Unable to parse enclosure file size")]
    ParseFileSize,
    #[error("Episode has no published at date")]
    NoPublishedAt,
    #[error("Unable to parse episode published at date")]
    ParsePublishedAt,
    #[error("Unable to parse episode duration")]
    ParseDuration,
    #[error("Unable to parse episode image")]
    ParseImage,
    #[error("Unable to parse episode number")]
    ParseEpisode,
    #[error("Unable to parse episode season number")]
    ParseSeason,
    #[error("Unable to parse episode type")]
    ParseKind,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_conversion() {
        // Arrange
        let source = PodcastFeed::example();

        // Act
        let rss = PodcastToRss::execute(source.clone());

        // Assert
        let xml = rss.to_string();
        assert_snapshot!(xml);

        // Act
        let result = PodcastFromRss::execute(rss, "test");

        // Assert
        let feed = result.assert_ok();
        assert_yaml_snapshot!(feed);
        assert_eq!(feed, source);
    }
}
