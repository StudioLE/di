use crate::prelude::*;

#[component]
pub fn PodcastPage(id: String) -> Element {
    let context = PodcastsContext::consume();
    if *context.loading.read() {
        return rsx! {
            "Loading..."
        };
    }
    let Some(feed) = context.podcasts.get(&id) else {
        return rsx! {
            "Unable to find podcast: {id}"
        };
    };
    rsx! {
        header { class: "media",
            figure { class: "media-left",
                p { class: "image is-128x128",
                    if let Some(url) = &feed.podcast.image {
                        img { src: "{url}" }
                    }
                }
            }
            div {
                class: "media-content",
                style: "align-self: center;",
                p { class: "title",
                    "{feed.podcast.title} "
                }
                p { class: "subtitle",
                    "{feed.episodes.len()} episodes · {feed.podcast.id}"
                }
            }
        }
        for episode in feed.episodes.iter() {
            div { class: "block item",
                Link {
                    to: Route::Episode { podcast_id: feed.podcast.id.clone(), episode_id: episode.id },
                    article { class: "media",
                        figure { class: "media-left",
                            p { class: "image is-64x64",
                                if let Some(url) = &episode.image {
                                    img { src: "{url}" }
                                } else {
                                    if let Some(url) = &feed.podcast.image {
                                        img { src: "{url}" }
                                    }
                                }
                            }
                        }
                        div {
                            class: "media-content",
                            style: "align-self: center;",
                            p { class: "title",
                                "{episode.title} "
                            }
                            p { class: "subtitle",
                                "{episode.published_at.format(\"%-d %B %Y\")}"
                                if episode.season.is_some() || episode.episode.is_some() {
                                    " · "
                                }
                                if let Some(season) = &episode.season {
                                    "S{season:02}"
                                }
                                if let Some(number) = &episode.episode {
                                    "E{number:02}"
                                }
                                if let Some(duration) = &episode.source_duration {
                                    " · {duration}s"
                                }
                                if let Some(kind) = episode.kind {
                                    if kind != EpisodeKind::Full {
                                        " · {kind}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
