use crate::prelude::*;

#[component]
pub fn PodcastPage(slug: Slug) -> Element {
    let slug_clone = slug.clone();
    let resource = use_resource(move || {
        let slug_clone = slug_clone.clone();
        async move { get_podcast(slug_clone).await }
    });
    match (*resource.read()).clone() {
        None => Loading(),
        Some(Err(error)) => Err(error.into()),
        Some(Ok(None)) => NoPodcast(NoPodcastProps { slug }),
        Some(Ok(Some((podcast, episodes)))) => Podcast(PodcastProps { podcast, episodes }),
    }
}

#[component]
fn Loading() -> Element {
    rsx! {
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
    }
}

#[component]
fn NoPodcast(slug: Slug) -> Element {
    rsx! {
        Page {
                title: "Podcast not found",
                subtitle: "404",
                MediaObject {
                    title: "Unable to find podcast",
                    subtitle: "{slug}",
                    image_size: ImageSize::_128,
                    icon: "fa-triangle-exclamation",
                }
            }
    }
}

#[component]
fn Podcast(podcast: PodcastPartial, episodes: Vec<EpisodePartial>) -> Element {
    let subtitle = format!("{} episodes · {}", episodes.len(), podcast.slug);
    rsx! {
        Page {
            title: podcast.title.clone(),
            subtitle: subtitle.clone(),
            MediaObject {
                title: podcast.title.clone(),
                subtitle: subtitle,
                image_src: podcast.image.clone().map(Url::from),
                image_size: ImageSize::_128,
                icon: "fa-image",
            }
            for episode in episodes {
                div { class: "block item",
                    Link {
                        to: Route::Episode { podcast_slug: podcast.slug.clone(), episode_key: episode.primary_key },
                        MediaObject {
                            title: episode.title.clone(),
                            subtitle: get_subtitle(episode.published_at,
                                episode.season,
                                episode.episode,
                                episode.source_duration,
                                episode.kind),
                            image_src: episode.image.clone().or_else(|| podcast.image.clone()).map(Url::from),
                            image_size: ImageSize::_64,
                            icon: "fa-image",
                        }
                    }
                }
            }
        }
    }
}

#[get("/api/podcasts/:slug")]
async fn get_podcast(
    slug: Slug,
) -> Result<Option<(PodcastPartial, Vec<EpisodePartial>)>, ServerFnError> {
    match SERVICES.metadata.get_podcast(slug).await {
        Ok(option) => Ok(option),
        Err(error) => {
            error!("{error:?}");
            Err(ServerFnError::new(error.to_string()))
        }
    }
}
