use crate::prelude::*;

#[component]
pub fn PodcastsPage() -> Element {
    let resource = use_resource(get_podcasts);
    let Some(result) = &*resource.read() else {
        return rsx! {
            "Loading..."
        };
    };
    match result {
        Ok(podcasts) => rsx! {
            for podcast in podcasts {
                div { class: "block",
                    Link {
                        to: Route::Podcast { id: podcast.id.clone() },
                        article { class: "media",
                            figure { class: "media-left",
                                p { class: "image is-96x96",
                                    if let Some(url) = &podcast.image_url {
                                        img { src: "{url}" }
                                    }
                                }
                            }
                            div {
                                class: "media-content",
                                style: "align-self: center;",
                                p { class: "subtitle is-5",
                                    "{podcast.title} "
                                    span { class: "tag",
                                        "{podcast.episodes.len()}"
                                    }
                                    " "
                                    span { class: "tag",
                                        "{podcast.id}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        Err(e) => rsx! {
            "{e}"
        },
    }
}

#[get("/api/podcasts")]
pub async fn get_podcasts() -> Result<Vec<Podcast>, ServerFnError> {
    let services = ServiceProvider::create()
        .await
        .expect("ServiceProvider should not fail");
    let command = ListCommand::new(services.paths, services.metadata);
    command
        .execute()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
