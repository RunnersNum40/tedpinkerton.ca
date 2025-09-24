use crate::{Logo, Route, StorageTheme, Theme};
use dioxus::prelude::*;

#[component]
fn NavItemLink(label: String, route: Route, enabled: bool) -> Element {
    let current: Route = use_route();

    rsx! {
        if enabled {
            Link {
                class: "nav-link",
                active_class: "active",
                to: route.clone(),
                aria_label: "Switch to {label} page",
                "{label}"
                if current == route {
                    span { class: "sr-only", " (current page)" }
                }
            }
        } else {
            span {
                class: "nav-link opacity-50 cursor-not-allowed",
                aria_disabled: "true",
                title: "Coming soon",
                "{label}"
                span { class: "sr-only", " (coming soon)" }
            }
        }
    }
}

#[component]
fn NavBar() -> Element {
    let links = [
        ("Home", Route::Home {}, true),
        ("Blog", Route::Blog {}, false),
        ("Projects", Route::Projects {}, false),
        ("Art", Route::Art {}, false),
        ("Contact", Route::Contact {}, true),
        ("Resume", Route::Resume {}, true),
    ];

    rsx! {
        nav { id: "navbar",
            a { href: "#main", class: "sr-only", "Skip to content" }
            div { class: "nav-inner",
                div { class: "nav-home",
                    Link { class: "nav-home-link", to: Route::Home {},
                        Logo {}
                        span { "tedpinkerton.ca" }
                    }
                }
                div { class: "nav-right",
                    div { class: "nav-links",
                        for (label, route, enabled) in links.iter() {
                            NavItemLink { label: *label, route: route.clone(), enabled: *enabled }
                        }
                    }
                    ThemeToggle {}
                }
            }
        }
    }
}

#[component]
fn ThemeToggle() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let mut stored_setting = use_context::<Signal<StorageTheme>>();

    let aria_label = match theme() {
        Theme::Light => "Switch to dark theme",
        Theme::Dark => "Switch to light theme",
    };
    let label = match theme() {
        Theme::Light => "Dark",
        Theme::Dark => "Light",
    };

    rsx! {
        button {
            id: "theme-toggle",
            class: "btn icon-btn",
            r#type: "button",
            "aria-label": aria_label,
            onclick: move |_| {
                let next = theme().toggle();
                stored_setting.set(Some(next));
            },
            span { class: "sr-only", "{aria_label}" }
            span { aria_hidden: "true", "{label}" }
        }
    }
}

#[component]
pub fn Header() -> Element {
    rsx! { NavBar {} }
}
