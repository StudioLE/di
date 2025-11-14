use crate::prelude::*;

/// A navigation drawer.
/// - `https://m3.material.io/components/navigation-drawer/overview`
#[component]
pub fn DrawerComponent() -> Element {
    rsx! {
        aside { style: "width: 250px; padding: 1.375em 1.5em;",
            Menu {
                lists: vec![
                    MenuListProps {
                        label: "Menu".to_owned(),
                        routes: vec![Route::Index, Route::AddPodcast, Route::Settings]
                    }
                ]
            }
        }
    }
}
