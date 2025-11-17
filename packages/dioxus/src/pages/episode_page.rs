use crate::prelude::*;
use html2text::config::plain;

#[component]
pub fn EpisodePage(podcast_id: String, episode_id: Uuid) -> Element {
    let context = PodcastsContext::consume();
    if *context.loading.read() {
        return rsx! {
            Page {
                title: "Loading...",
                SkeletonMediaObject {
                    image_size: ImageSize::_128,
                    icon: "fa-image",
                }
                for _i in 0..5 {
                    div { class: "block item pulse-animation",
                        a {
                            SkeletonMediaObject {
                                image_size: ImageSize::_64,
                                icon: "fa-image",
                            }
                        }
                    }
                }
            }
        };
    }
    let Some(feed) = context.podcasts.get(&podcast_id) else {
        return rsx! {
            Page {
                title: "Podcast not found",
                subtitle: "404",
                MediaObject {
                    title: "Unable to find podcast",
                    subtitle: "{podcast_id}",
                    image_size: ImageSize::_128,
                    icon: "fa-triangle-exclamation",
                }
            }
        };
    };
    let Some(episode) = feed
        .episodes
        .iter()
        .find(|episode| episode.id == episode_id)
    else {
        return rsx! {
            Page {
                title: "Episode not found",
                subtitle: "404",
                MediaObject {
                    title: "Unable to find episode",
                    subtitle: "{podcast_id} · {episode_id}",
                    image_size: ImageSize::_128,
                    icon: "fa-triangle-exclamation",
                }
            }
        };
    };
    let description = episode.get_description();
    let subtitle = episode.get_subtitle();
    let image = episode
        .get_image_url()
        .or_else(|| feed.podcast.get_image_url());
    rsx! {
        Page {
            title: episode.title.clone(),
            subtitle: subtitle.clone(),
            div { class: "block",
                MediaObject {
                    title: episode.title.clone(),
                    subtitle: subtitle,
                    image_src: image,
                    image_size: ImageSize::_128,
                    icon: "fa-image",
                }
                if let Some(description) = description {
                    article {
                        pre {
                            "{description}"
                        }
                    }
                }
            }
        }
    }
}
