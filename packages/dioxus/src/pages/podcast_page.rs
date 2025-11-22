use crate::prelude::*;

#[component]
pub fn PodcastPage(id: String) -> Element {
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
    let Some(feed) = context.podcasts.get(&id) else {
        return rsx! {
            Page {
                title: "Podcast not found",
                subtitle: "404",
                MediaObject {
                    title: "Unable to find podcast",
                    subtitle: "{id}",
                    image_size: ImageSize::_128,
                    icon: "fa-triangle-exclamation",
                }
            }
        };
    };
    let subtitle = format!("{} episodes · {}", feed.episodes.len(), feed.podcast.slug);
    rsx! {
        Page {
            title: feed.podcast.title.clone(),
            subtitle: subtitle.clone(),
            MediaObject {
                title: feed.podcast.title.clone(),
                subtitle: subtitle,
                image_src: feed.podcast.get_image_url(),
                image_size: ImageSize::_128,
                icon: "fa-image",
            }
            for episode in feed.episodes.iter() {
                div { class: "block item",
                    Link {
                        to: Route::Episode { podcast_slug: feed.podcast.slug.clone(), episode_key: episode.primary_key },
                        MediaObject {
                            title: episode.title.clone(),
                            subtitle: episode.get_subtitle(),
                            image_src: episode.get_image_url().or_else(|| feed.podcast.get_image_url()),
                            image_size: ImageSize::_64,
                            icon: "fa-image",
                        }
                    }
                }
            }
        }
    }
}
