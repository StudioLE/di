use crate::prelude::*;

/// The classic button, in different colors, sizes, and states.
///
/// An implementation of the [Bulma button element](https://bulma.io/documentation/elements/button/).
#[component]
pub fn Button(
    class: Option<String>,
    style: Option<String>,
    route: Option<Route>,
    color: Option<ButtonColor>,
    size: Option<ButtonSize>,
    fullwidth: Option<bool>,
    children: Element,
) -> Element {
    let class = get_class(class, color, size, fullwidth);
    if let Some(route) = route {
        return rsx! {
            Link { class: "{class}",
                style: style,
                to: route,
                { children }
            }
        };
    }
    rsx! {
        a { class: "{class}",
            style: style,
            { children }
        }
    }
}

fn get_class(
    class: Option<String>,
    color: Option<ButtonColor>,
    size: Option<ButtonSize>,
    fullwidth: Option<bool>,
) -> String {
    let mut classes = Vec::new();
    classes.push("button".to_owned());
    if let Some(color) = color {
        classes.push(color.get_class());
    }
    if let Some(size) = size {
        classes.push(size.get_class());
    }
    if fullwidth == Some(true) {
        classes.push("is-fullwidth".to_owned());
    }
    if let Some(class) = class {
        classes.push(class);
    }
    classes.join(" ")
}

/// - <https://bulma.io/documentation/elements/button/#colors>
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonColor {
    White,
    Light,
    Dark,
    Black,
    Text,
    Ghost,

    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
}

impl ButtonColor {
    #[must_use]
    fn get_class(self) -> String {
        let str = match self {
            ButtonColor::White => "is-white",
            ButtonColor::Light => "is-light",
            ButtonColor::Dark => "is-dark",
            ButtonColor::Black => "is-black",
            ButtonColor::Text => "is-text",
            ButtonColor::Ghost => "is-ghost",
            ButtonColor::Primary => "is-primary",
            ButtonColor::Link => "is-link",
            ButtonColor::Info => "is-info",
            ButtonColor::Success => "is-success",
            ButtonColor::Warning => "is-warning",
            ButtonColor::Danger => "is-danger",
        };
        str.to_owned()
    }
}

/// - <https://bulma.io/documentation/elements/button/#sizes>
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonSize {
    Small,
    Normal,
    Medium,
    Large,
}

impl ButtonSize {
    #[must_use]
    pub fn get_class(&self) -> String {
        let str = match self {
            ButtonSize::Small => "is-small",
            ButtonSize::Normal => "is-normal",
            ButtonSize::Medium => "is-medium",
            ButtonSize::Large => "is-large",
        };
        str.to_owned()
    }
}