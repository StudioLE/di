use crate::prelude::*;

#[component]
pub fn EpisodePage(podcast_id: String, episode_id: String) -> Element {
    let context = PodcastsContext::consume();
    if *context.loading.read() {
        return rsx! {
            "Loading..."
        };
    }
    let Some(podcast) = context.podcasts.get(&podcast_id) else {
        return rsx! {
            "Unable to find podcast: {podcast_id}"
        };
    };
    let Some(episode) = podcast
        .episodes
        .iter()
        .find(|episode| episode.id == episode_id)
    else {
        return rsx! {
            "Unable to find episode: {episode_id}"
        };
    };
    rsx! {
        div { class: "block",
            header { class: "media",
                figure { class: "media-left",
                    p { class: "image is-128x128",
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
            article {
                pre {
                    "{episode.description}"
                }
            }
        }
    }
}
