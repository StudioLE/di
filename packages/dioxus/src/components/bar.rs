use crate::prelude::*;

/// A [Material Design 3 app bar](https://m3.material.io/components/app-bars/overview).
#[component]
pub fn Bar() -> Element {
    let current: Route = use_route();
    let info = current.get_info();
    let breadcrumbs = current.get_info().breadcrumbs;
    let previous = if breadcrumbs.len() > 1 {
        breadcrumbs.get(breadcrumbs.len() - 2)
    } else {
        None
    };
    rsx! {
        header { style: "
margin: var(--bulma-block-spacing) 0;
display: flex;
align-items: center;
gap: 1rem;",
            if let Some(previous) = previous {
                div { style: "",
                    Link { style: "",
                        to: previous.clone(),
                        span {
                            class: "icon is-medium",
                            i { class: "fas fa-arrow-left fa-lg" }
                        }
                    }
                }
            }
            div {
                p { class: "title",
                    "{info.title} "
                }
                p { class: "subtitle",
                    "This is a subtitle"
                }
            }
        }
    }
}
