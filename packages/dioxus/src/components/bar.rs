use crate::prelude::*;

/// A [Material Design 3 app bar](https://m3.material.io/components/app-bars/overview).
#[component]
pub fn AppBar(title: String, subtitle: Option<String>) -> Element {
    let current: Route = use_route();
    let previous = current.get_info().previous;
    rsx! {
        header { style: "
            position: fixed;
            left: 0;
            right: 0;
            top: 0;
            z-index: 1;
            background-color: var(--bulma-body-background-color)",
            class: "container is-max-tablet",
            div { style: "
                margin: var(--bulma-block-spacing) 0;
                display: flex;
                align-items: center;
                gap: 1rem;",
                if let Some(previous) = previous {
                    div {
                        Button {
                            route: previous.clone(),
                            color: ButtonColor::Text,
                            style: "text-decoration: none;",
                            Icon {
                                class: "fa-arrow-left",
                                container_size: IconContainerSize::Medium,
                                size: IconSize::Large,
                            }
                        }
                    }
                }
                div {
                    p { class: "title",
                        "{title} "
                    }
                    if let Some(subtitle) = subtitle {
                        p { class: "subtitle",
                            "{subtitle}"
                        }
                    }
                }
            }
        }
    }
}
