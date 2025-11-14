use crate::prelude::*;
use Route::*;

/// Properties for [`Menu`]
#[derive(Clone, Debug, PartialEq, Props)]
pub struct MenuProps {
    lists: Vec<MenuListProps>,
}

/// A simple menu, for any type of vertical navigation.
///
/// An implementation of the [Bulma menu component](https://bulma.io/documentation/components/menu/).
///
/// The design is similar to the [Material Design 3 navigation drawer](https://m3.material.io/components/navigation-drawer/overview).
#[component]
pub fn Menu(props: MenuProps) -> Element {
    rsx! {
        aside { class: "menu",
            for list in props.lists {
                MenuList {
                    label: list.label,
                    routes: list.routes
                }
            }
        }
    }
}

/// A list of menu items with a label.
#[derive(Clone, Debug, PartialEq, Props)]
pub struct MenuListProps {
    pub label: String,
    pub routes: Vec<Route>,
}

/// A list of [`MenuItem`]s
#[component]
fn MenuList(props: MenuListProps) -> Element {
    rsx! {
        p { class: "menu-label", "{props.label}" }
        ul { class: "menu-list",
            for route in props.routes {
                MenuItem { route }
            }
        }
    }
}

#[component]
fn MenuItem(route: Route) -> Element {
    let current: Route = use_route();
    let is_active = current.get_info().breadcrumbs.contains(&route);
    let info = route.get_info();
    rsx! {
        li {
            Link {
                to: route,
                class: if is_active { "is-active" } else { "" },
                span { class: "icon has-text-grey-dark",
                    i { class: info.get_icon_classes() }
                }
                span { "{info.title}" }
            }
        }
    }
}
