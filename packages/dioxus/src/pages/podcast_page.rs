use crate::prelude::*;

#[component]
pub fn PodcastPage(id: String) -> Element {
    let context = PodcastsContext::consume();
    if *context.loading.read() {
        return rsx! {
            "Loading..."
        };
    }
    let Some(podcast) = context.podcasts.get(&id) else {
        return rsx! {
            "Unable to find podcast: {id}"
        };
    };
    rsx! {
        header { class: "media",
            figure { class: "media-left",
                p { class: "image is-128x128",
                    if let Some(url) = &podcast.image_url {
                        img { src: "{url}" }
                    }
                }
            }
            div {
                class: "media-content",
                style: "align-self: center;",
                p { class: "title",
                    "{podcast.title} "
                }
                p { class: "subtitle",
                    "{podcast.episodes.len()} episodes · {podcast.id}"
                }
            }
        }
        for episode in podcast.episodes.iter() {
            div { class: "block",
                Link {
                    to: Route::Episode { podcast_id: podcast.id.clone(), episode_id: episode.id.clone() },
                    article { class: "media",
                        figure { class: "media-left",
                            p { class: "image is-64x64",
                                if let Some(url) = &episode.image_url {
                                    img { src: "{url}" }
                                } else {
                                    if let Some(url) = &podcast.image_url {
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
                                if episode.season.is_some() || episode.number.is_some() {
                                    " · "
                                }
                                if let Some(season) = &episode.season {
                                    "S{season:02}"
                                }
                                if let Some(number) = &episode.number {
                                    "E{number:02}"
                                }
                                if let Some(duration) = &episode.duration {
                                    " · {duration}s"
                                }
                                if episode.episode_type != EpisodeType::Full {
                                    " · {episode.episode_type}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
