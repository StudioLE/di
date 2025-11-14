use crate::prelude::*;
use Route::*;

/// Simple responsive horizontal navigation tabs, with different styles.
///
/// An implementation of the [Bulma tabs component](https://bulma.io/documentation/components/tabs/).
///
/// The design is similar to the [Material Design 3 navigation bar](https://m3.material.io/components/navigation-bar/overview).
#[component]
pub fn Tabs(routes: Vec<Route>) -> Element {
    rsx! {
        div { class: "tabs is-centered is-large is-fullwidth",
            ul {
                for route in routes {
                    Tab { route }
                }
            }
        }
    }
}

#[component]
fn Tab(route: Route) -> Element {
    let current: Route = use_route();
    let is_active = current.get_info().breadcrumbs.contains(&route);
    let info = route.get_info();
    rsx! {
        li { class: if is_active { "is-active" } else { "" },
            Link {
                to: route,
                span { class: "icon",
                    i { class: info.get_icon_classes() }
                }
                span { "{info.title}" }
            }
        }
    }
}
