use crate::prelude::*;

#[component]
pub fn IndexPage() -> Element {
    let context = PodcastsContext::consume();
    if *context.loading.read() {
        return rsx! {
            "Loading..."
        };
    }
    let podcasts = context.podcasts.read();
    if podcasts.is_empty() {
        return rsx! {
            "No podcasts found"
        };
    }
    rsx! {
        for podcast in podcasts.values() {
            div { class: "block item",
                Link {
                    to: Route::Podcast { id: podcast.id.clone() },
                    article { class: "media",
                        figure { class: "media-left",
                            p { class: "image is-64x64",
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
                }
            }
        }
    }
}
