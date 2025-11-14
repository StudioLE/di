use crate::prelude::*;

#[component]
pub fn App() -> Element {
    SettingsContext::create();
    PodcastsContext::create();
    rsx! {
        Router::<Route> {}
    }
}

#[component]
pub fn Layout() -> Element {
    rsx! {
        HeadComponent {}
        div { style: " height: 100vh; display: flex; flex-direction: column;",
            div { style: "flex: 1; min-height: 0; display: flex;",
                Drawer {
                    lists: vec![
                        MenuListProps {
                            label: "Menu".to_owned(),
                            routes: vec![Route::Index, Route::AddPodcast, Route::Settings]
                        }
                    ]
                },
                div { style: "flex: 1; display: flex; position: relative;",
                    div { style: "flex: 1; overflow-y: auto;",
                        main { class: "container is-max-tablet",
                            HeaderComponent {}
                            Outlet::<Route> {}
                        }
                    }
                    div { style: "position: absolute; bottom: 0; right: 0;",
                        FloatingActions {
                            routes: vec![Route::AddPodcast]
                        }
                    }
                }
            }
            footer { style: "flex-shrink: 0;",
                Tabs {
                    routes: vec![Route::Index, Route::AddPodcast, Route::Settings]
                }
            }
        }
    }
}
