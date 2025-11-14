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
        aside { style: "width: 250px; padding: 1.375em 1.5em;",
            Menu {
                lists: props.lists
            }
        }
    }
}
