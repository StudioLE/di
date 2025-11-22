use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Index,
    #[route("/podcasts/:id")]
    Podcast { id: String },
    #[route("/podcasts/:podcast_slug/:episode_key")]
    Episode {
        podcast_slug: String,
        episode_key: u32,
    },
    #[route("/settings")]
    Settings,
    #[route("/settings/player")]
    PlayerSettings,
    #[route("/add")]
    AddPodcast,
}

impl Route {
    #[must_use]
    pub fn get_info(&self) -> RouteInfo {
        match self {
            Route::Index => RouteInfo {
                title: "Podcasts".to_owned(),
                icon: "fa-podcast".to_owned(),
                previous: None,
                breadcrumbs: vec![Route::Index],
                path: "/".to_owned(),
            },
            Route::Podcast { id } => RouteInfo {
                title: "Podcast".to_owned(),
                icon: "fa-user".to_owned(),
                previous: Some(Route::Index),
                breadcrumbs: vec![Route::Index, Route::Podcast { id: id.clone() }],
                path: format!("/podcasts/{id}"),
            },
            Route::Episode {
                podcast_slug,
                episode_key,
            } => RouteInfo {
                title: "Episode".to_owned(),
                icon: "fa-user".to_owned(),
                previous: Some(Route::Podcast {
                    id: podcast_slug.clone(),
                }),
                breadcrumbs: vec![
                    Route::Index,
                    Route::Episode {
                        podcast_slug: podcast_slug.clone(),
                        episode_key: *episode_key,
                    },
                ],
                path: format!("/podcasts/{podcast_slug}/{episode_key}"),
            },
            Route::Settings => RouteInfo {
                title: "Settings".to_owned(),
                icon: "fa-cog".to_owned(),
                previous: Some(Route::Index),
                breadcrumbs: vec![Route::Settings],
                path: "/settings".to_owned(),
            },
            Route::PlayerSettings => RouteInfo {
                title: "Player".to_owned(),
                icon: "fa-play".to_owned(),
                previous: Some(Route::Settings),
                breadcrumbs: vec![Route::Settings, Route::PlayerSettings],
                path: "/settings/player".to_owned(),
            },
            Route::AddPodcast => RouteInfo {
                title: "Add Podcast".to_owned(),
                icon: "fa-plus".to_owned(),
                previous: Some(Route::Index),
                breadcrumbs: vec![Route::AddPodcast],
                path: "/add".to_owned(),
            },
        }
    }
}

#[component]
fn Index() -> Element {
    IndexPage()
}

#[component]
fn Podcast(id: String) -> Element {
    PodcastPage(PodcastPageProps { id })
}

#[component]
fn Episode(podcast_slug: String, episode_key: u32) -> Element {
    EpisodePage(EpisodePageProps {
        podcast_slug,
        episode_key,
    })
}

#[component]
fn Settings() -> Element {
    SettingsPage()
}

#[component]
fn PlayerSettings() -> Element {
    PlayerSettingsPage()
}

#[component]
fn AddPodcast() -> Element {
    AddPodcastPage()
}
