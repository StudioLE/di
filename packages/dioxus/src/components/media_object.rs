use crate::prelude::*;

/// The famous media object prevalent in social media interfaces, but useful in any context
///
/// An implementation of the [Bulma media object](https://bulma.io/documentation/layout/media-object/).
#[component]
pub fn MediaObject(
    title: String,
    subtitle: Option<String>,
    image_size: ImageSize,
    image_src: Option<UrlWrapper>,
    icon: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "media",
            figure { class: "media-left",
                if let Some(src) = image_src {
                    p { class: "image {image_size.get_class()}",
                        img { src: "{src}" }
                    }
                } else {
                    div { class: "image {image_size.get_class()}",
                        div { style: "
                            height: 100%;
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            background-color: var(--bulma-grey-darker);
                            color: var(--bulma-black-ter);",
                            if let Some(icon) = icon {
                                Icon {
                                    class: icon,
                                    size: IconSize::ExtraExtraLarge
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "media-content",
                style: "margin-inline-end: var(--bulma-media-spacing); align-self: center; display: flex; align-items: center; ",
                div { style: "flex: 1;",
                    p { class: "title",
                        "{title}"
                    }
                    if let Some(subtitle) = subtitle {
                        p { class: "subtitle",
                            "{subtitle}"
                        }
                    }
                }
                div { style: "flex: 0;",
                    { children }
                }
            }
        }
    }
}

/// A skeleton for [`MediaObject`].
#[component]
pub fn SkeletonMediaObject(image_size: ImageSize, icon: Option<String>) -> Element {
    rsx! {
        div { class: "media",
            style: "opacity: 0.1",
            figure { class: "media-left",
                div { class: "image {image_size.get_class()}",
                    div { style: "
                        height: 100%;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        background-color: var(--bulma-grey-darker);
                        color: var(--bulma-black-ter);",
                        if let Some(icon) = icon {
                            Icon {
                                class: icon,
                                size: IconSize::ExtraExtraLarge
                            }
                        }
                    }
                }
            }
            div {
                class: "media-content",
                style: "align-self: center;",
                div { class: "title",
                    div {
                        style: "
                        margin-bottom: 0.1em;
                        height: 0.9em;
                        width: 65%;
                        background-color: var(--bulma-title-color);
                        border-radius: 0.25rem;"
                    }
                }
                div { class: "subtitle",
                    div {
                        style: "
                        margin-top: 0.1em;
                        height: 0.9em; width: 55%;
                        background-color: var(--bulma-subtitle-color);
                        border-radius: 0.25rem;"
                    }
                }
            }
        }
    }
}
