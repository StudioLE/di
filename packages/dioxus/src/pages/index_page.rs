use crate::prelude::*;

#[component]
pub fn IndexPage() -> Element {
    let resource = use_resource(move || async move { get_podcasts().await });
    match (*resource.read()).clone() {
        None => Loading(),
        Some(Err(error)) => Err(error.into()),
        Some(Ok(podcasts)) if podcasts.is_empty() => NoPodcasts(),
        Some(Ok(podcasts)) => Podcasts(PodcastsProps { podcasts }),
    }
}

#[component]
fn Loading() -> Element {
    rsx! {
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
    }
}

#[component]
fn NoPodcasts() -> Element {
    rsx! {
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
    }
}

#[component]
fn Podcasts(podcasts: Vec<PodcastPartial>) -> Element {
    rsx! {
        Page {
            title: "Podcasts",
            subtitle: "{podcasts.len()} podcasts",
            for podcast in podcasts {
                div { class: "block item",
                    Link {
                        to: Route::Podcast { slug: podcast.slug.clone() },
                        MediaObject {
                            title: podcast.title.clone(),
                            subtitle: "{podcast.episodes_count} episodes · {podcast.slug}",
                            image_src: podcast.image.clone().map(Url::from),
                            image_size: ImageSize::_64,
                            icon: "fa-image",
                        }
                    }
                }
            }
        }
    }
}

#[get("/api/podcasts")]
async fn get_podcasts() -> Result<Vec<PodcastPartial>, ServerFnError> {
    match METADATA.get_podcasts().await {
        Ok(podcasts) => Ok(podcasts),
        Err(error) => {
            error!("{error:?}");
            Err(ServerFnError::new(error.to_string()))
        }
    }
}
