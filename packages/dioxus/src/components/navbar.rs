use crate::prelude::*;

/// A [Material Design 3 navigation bar](https://m3.material.io/components/navigation-bar/overview).
#[component]
pub fn NavigationBar() -> Element {
    rsx! {
        footer { style: "
            position: fixed;
            left: 0;
            right: 0;
            bottom: 0;
            z-index: 3;
            background-color: var(--overlay-bg);",
            Tabs {
                routes: vec![Route::Index, Route::AddPodcast, Route::Settings],
                link_style: "display: flex; flex-direction: column;".to_owned(),
                icon_size: IconSize::ExtraLarge,
                icon_container_size: IconContainerSize::Large,
            }
        }
    }
}
