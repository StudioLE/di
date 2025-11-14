use crate::prelude::*;

/// A stack of [Material Design 3 floating action button](https://m3.material.io/components/floating-action-button/overview).
#[component]
pub fn FloatingActions(routes: Vec<Route>) -> Element {
    rsx! {
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

/// A [Material Design 3 floating action button](https://m3.material.io/components/floating-action-button/overview).
#[component]
pub fn FloatingAction(route: Route, is_large: bool) -> Element {
    let info = route.get_info();
    rsx! {
        Link { style: "width: fit-content;",
            to: route,
            class: get_button_classes(is_large),
            span {
                class: "icon",
                i { class: info.get_icon_classes() }
            }
        }
    }
}

fn get_button_classes(is_large: bool) -> String {
    let mut output = "button is-primary".to_owned();
    if is_large {
        output.push_str(" is-large");
    }
    output
}
