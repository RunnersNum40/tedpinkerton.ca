use crate::{Route, StorageTheme, Theme};
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let mut stored_setting = use_context::<Signal<StorageTheme>>();

    let label = match theme() {
        Theme::Light => "Light",
        Theme::Dark => "Dark",
    };

    rsx! {
        nav { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Blog {}, "Blog" }
            Link { to: Route::Projects {}, "Projects" }
            Link { to: Route::Art {}, "Art" }
            Link { to: Route::About {}, "About" }

            button {
                id: "theme-toggle", r#type: "button",
                "aria-label": "Toggle theme",
                onclick: move |_| {
                    stored_setting.set(theme().toggle().into());
                },
                "{label}"
            }
        }
    }
}

