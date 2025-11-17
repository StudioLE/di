use crate::prelude::*;
use rss::extension::ExtensionMap;
use rss::extension::itunes::{ITunesCategory, ITunesItemExtension};

pub struct PodcastToRss;

impl PodcastToRss {
    pub fn execute(feed: PodcastFeed) -> RssChannel {
        let mut rss = podcast_to_rss(feed.podcast.clone());
        rss.items = feed.episodes.into_iter().map(episode_to_rss).collect();
        rss
    }
}

fn podcast_to_rss(podcast: PodcastInfo) -> RssChannel {
    RssChannel {
        title: podcast.title,
        link: podcast.link.unwrap_or_default(),
        description: podcast.description.clone(),
        language: podcast.language.clone(),
        copyright: podcast.copyright.clone(),
        itunes_ext: Some(ITunesChannelExtension {
            author: podcast.author,
            categories: podcast
                .categories
                .into_iter()
                .map(|category| ITunesCategory {
                    text: category.category,
                    subcategory: category.sub_category.map(|sub| {
                        Box::new(ITunesCategory {
                            text: sub,
                            subcategory: None,
                        })
                    }),
                })
                .collect(),
            image: podcast.image,
            explicit: Some(podcast.explicit.to_string()),
            new_feed_url: podcast.new_feed_url,
            r#type: podcast.kind.map(|kind| kind.to_string()),
            ..ITunesChannelExtension::default()
        }),
        ..RssChannel::default()
    }
}

fn episode_to_rss(episode: EpisodeInfo) -> RssItem {
    RssItem {
        title: Some(episode.title),
        link: None,
        description: episode.description,
        author: None,
        categories: Vec::new(),
        comments: None,
        enclosure: Some(RssEnclosure {
            url: episode.source_url.clone(),
            length: episode.source_file_size.to_string(),
            mime_type: episode.source_content_type.clone(),
        }),
        guid: Some(RssGuid {
            value: episode.id.to_string(),
            permalink: false,
        }),
        pub_date: Some(episode.published_at.to_rfc2822()),
        source: None,
        content: None,
        extensions: ExtensionMap::default(),
        itunes_ext: Some(ITunesItemExtension {
            duration: episode.source_duration.map(|d| d.to_string()),
            explicit: episode.explicit.map(|explicit| explicit.to_string()),
            image: episode.image.as_ref().map(ToString::to_string),
            episode: episode.episode.map(|n| n.to_string()),
            season: episode.season.map(|s| s.to_string()),
            episode_type: episode.kind.map(|kind| kind.to_string()),
            ..Default::default()
        }),
        dublin_core_ext: None,
    }
}
