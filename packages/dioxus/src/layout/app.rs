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
        Drawer {
            lists: vec![
                MenuListProps {
                    label: "Menu".to_owned(),
                    routes: vec![Route::Index, Route::AddPodcast, Route::Settings]
                }
            ]
        },
        Outlet::<Route> {}
        FloatingActions {
            routes: vec![Route::AddPodcast]
        }
        NavigationBar {}
    }
}
