use crate::{Route, StorageTheme, Theme};
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let mut stored_setting = use_context::<Signal<StorageTheme>>();

    let label = match theme().toggle() {
        Theme::Light => "Light",
        Theme::Dark => "Dark",
    };

    rsx! {
        nav { id: "navbar",
            a { href: "#main", class: "sr-only", "Skip to content" }
            div { class: "nav-inner",
                div { class: "nav-home",
                    Link { to: Route::Home {}, "tedpinkerton.ca" }
                }
                div { class: "nav-links",
                    Link { class: "nav-link", to: Route::Home {}, "Home" }
                    Link { class: "nav-link", to: Route::Blog {}, "Blog" }
                    Link { class: "nav-link", to: Route::Projects {}, "Projects" }
                    Link { class: "nav-link", to: Route::Art {}, "Art" }
                    Link { class: "nav-link", to: Route::About {}, "About" }

                    button {
                        id: "theme-toggle",
                        class: "btn icon-btn",
                        r#type: "button",
                        "aria-label": "Toggle theme",
                        onclick: move |_| { stored_setting.set(theme().toggle().into()); },
                        span { class: "sr-only", "Toggle theme" }
                        span { "{label}"}
                    }
                }
            }
        }
    }
}
