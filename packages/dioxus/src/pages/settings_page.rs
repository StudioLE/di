use crate::prelude::*;

#[component]
pub fn SettingsPage() -> Element {
    rsx! {
        section { class: "section",
            Menu {
                lists: vec![
                    MenuListProps {
                        label: "General".to_owned(),
                        routes: vec![Route::PlayerSettings]
                    },
                    MenuListProps {
                        label: "Player".to_owned(),
                        routes: vec![Route::PlayerSettings, Route::PlayerSettings, Route::PlayerSettings]
                    }
                ]
            }
        }
    }
}
