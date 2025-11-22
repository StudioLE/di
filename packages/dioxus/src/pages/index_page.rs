use crate::prelude::*;

#[component]
pub fn IndexPage() -> Element {
    let context = PodcastsContext::consume();
    if *context.loading.read() {
        return rsx! {
            Page {
                title: "Loading...",
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
    let podcasts = context.podcasts.read();
    if podcasts.is_empty() {
        return rsx! {
            Page {
                title: "Podcasts",
                subtitle: "0 podcasts",
                div { class: "block item",
                    Link {
                        to: Route::AddPodcast,
                        MediaObject {
                            title: "Your collection is empty",
                            subtitle: "Add your first podcast to get started",
                            image_size: ImageSize::_64,
                            icon: "fa-plus",
                        }
                    }
                }
            }
        };
    }
    rsx! {
        Page {
            title: "Podcasts",
            subtitle: "{podcasts.len()} podcasts",
            for feed in podcasts.values() {
                div { class: "block item",
                    Link {
                        to: Route::Podcast { id: feed.podcast.slug.clone() },
                        MediaObject {
                            title: feed.podcast.title.clone(),
                            subtitle: "{feed.episodes.len()} episodes · {feed.podcast.slug}",
                            image_src: feed.podcast.get_image_url(),
                            image_size: ImageSize::_64,
                            icon: "fa-image",
                        }
                    }
                }
            }
        }
    }
}
