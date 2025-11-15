use crate::prelude::*;

/// Properties for [`Drawer`]
#[derive(Clone, Debug, PartialEq, Props)]
pub struct DrawerProps {
    lists: Vec<MenuListProps>,
}

/// A [Material Design 3 navigation drawer](https://m3.material.io/components/navigation-drawer/overview).
#[component]
pub fn Drawer(props: DrawerProps) -> Element {
    rsx! {
        aside { class: "drawer",
            style: "
        position: fixed;
        left: 0;
        top: 0;
        bottom: 0;
        z-index: 2;
        padding: 1.375em 1.5em;
        width: 250px;
        background-color: var(--overlay-bg);
        ",
            Menu {
                lists: props.lists
            }
        }
    }
}
