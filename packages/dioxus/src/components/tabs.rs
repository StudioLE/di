use crate::prelude::*;

/// Simple responsive horizontal navigation tabs, with different styles.
///
/// An implementation of the [Bulma tabs component](https://bulma.io/documentation/components/tabs/).
///
/// The design is similar to the [Material Design 3 navigation bar](https://m3.material.io/components/navigation-bar/overview).
#[component]
pub fn Tabs(
    routes: Vec<Route>,
    icon_size: Option<IconSize>,
    icon_container_size: Option<IconContainerSize>,
    link_style: Option<String>,
) -> Element {
    rsx! {
        div { class: "tabs is-centered is-fullwidth",
            ul {
                for route in routes {
                    Tab {
                        route,
                        icon_size,
                        link_style: link_style.clone(),
                    }
                }
            }
        }
    }
}

#[component]
fn Tab(
    route: Route,
    style: Option<String>,
    icon_size: Option<IconSize>,
    icon_container_size: Option<IconContainerSize>,
    link_style: Option<String>,
) -> Element {
    let current: Route = use_route();
    let is_active = current.get_info().breadcrumbs.contains(&route);
    let info = route.get_info();
    rsx! {
        li { class: if is_active { "is-active" } else { "" },
            Link {
                style: link_style,
                to: route,
                Icon { class: info.icon,
                    size: icon_size,
                    container_size: icon_container_size,
                }
                span { "{info.title}" }
            }
        }
    }
}
