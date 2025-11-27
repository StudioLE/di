use crate::prelude::*;
use html2text::config::plain;

#[component]
pub fn EpisodePage(podcast_slug: Slug, episode_key: u32) -> Element {
    let resource_podcast_slug = podcast_slug.clone();
    let resource = use_resource(move || {
        let resource_podcast_slug = resource_podcast_slug.clone();
        async move { get_episode(resource_podcast_slug, episode_key).await }
    });
    match (*resource.read()).clone() {
        None => Loading(),
        Some(Err(error)) => Err(error.into()),
        Some(Ok(None)) => NoEpisode(NoEpisodeProps {
            podcast_slug,
            episode_key,
        }),
        Some(Ok(Some(episode))) => Episode(EpisodeProps { episode }),
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
fn NoEpisode(podcast_slug: Slug, episode_key: u32) -> Element {
    rsx! {
            Page {
                title: "Episode not found",
                subtitle: "404",
                MediaObject {
                    title: "Unable to find episode",
                    subtitle: "{podcast_slug} · {episode_key}",
                    image_size: ImageSize::_128,
                    icon: "fa-triangle-exclamation",
                }
            }
    }
}

#[component]
fn Episode(episode: EpisodePartial) -> Element {
    let description = format_description(episode.description);
    let subtitle = get_subtitle(
        episode.published_at,
        episode.season,
        episode.episode,
        episode.source_duration,
        episode.kind,
    );
    // TODO: Fallback to podcast image?
    let image = get_image_url(episode.image.clone());
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

#[get("/api/podcasts/:podcast_slug/episode/:episode_key")]
async fn get_episode(
    podcast_slug: Slug,
    episode_key: u32,
) -> Result<Option<EpisodePartial>, ServerFnError> {
    match SERVICES
        .metadata
        .get_episode(podcast_slug, episode_key)
        .await
    {
        Ok(podcasts) => Ok(podcasts),
        Err(error) => {
            error!("{error:?}");
            Err(ServerFnError::new(error.to_string()))
        }
    }
}

fn format_description(description: Option<String>) -> Option<String> {
    let description = description?;
    if description.starts_with('<') {
        plain()
            .no_link_wrapping()
            .do_decorate()
            .link_footnotes(true)
            .string_from_read(description.as_bytes(), 1000)
            .ok()
    } else {
        Some(description)
    }
}
