use crate::prelude::*;

/// The famous media object prevalent in social media interfaces, but useful in any context
///
/// An implementation of the [Bulma media object](https://bulma.io/documentation/layout/media-object/).
#[component]
pub fn MediaObject(
    title: String,
    subtitle: Option<String>,
    image_size: ImageSize,
    image_src: Option<Url>,
) -> Element {
    rsx! {
        div { class: "media",
            figure { class: "media-left",
                p { class: "image {image_size.get_class()}",
                    if let Some(src) = image_src {
                        img { src: "{src}" }
                    }
                }
            }
            div {
                class: "media-content",
                style: "align-self: center;",
                p { class: "title",
                    "{title}"
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
