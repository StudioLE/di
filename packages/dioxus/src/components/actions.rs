use crate::prelude::*;

/// A stack of [Material Design 3 floating action button](https://m3.material.io/components/floating-action-button/overview).
#[component]
pub fn FloatingActions(routes: Vec<Route>) -> Element {
    rsx! {
        div { style: "position: fixed; right: 0; bottom: 60px;",
            div { style: "
                display: flex;
                margin: 1rem;
                flex-direction: column-reverse;
                align-items: center;
                flex: 0;
                gap: 1rem;",
                for (index, route) in routes.iter().enumerate() {
                    FloatingAction {
                        route: route.clone(),
                        is_large: index == 0
                    }
                }
            }
        }
    }
}

/// A [Material Design 3 floating action button](https://m3.material.io/components/floating-action-button/overview).
#[component]
pub fn FloatingAction(route: Route, is_large: bool) -> Element {
    let info = route.get_info();
    rsx! {
        Button { style: "width: fit-content;",
            route: route,
            color: ButtonColor::Primary,
            size: if is_large { Some(ButtonSize::Large) } else { None },
            Icon { class: info.icon }
        }
    }
}
