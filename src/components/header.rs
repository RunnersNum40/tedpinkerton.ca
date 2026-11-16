use crate::{Logo, Route};
use dioxus::prelude::*;

#[component]
fn NavItemLink(label: String, route: Route, enabled: bool) -> Element {
    let current: Route = use_route();
    let is_active = match (&route, &current) {
        (Route::Blog {}, Route::BlogPost { .. }) => true,
        _ => current == route,
    };

    rsx! {
        if enabled {
            Link {
                class: if is_active { "nav-link active" } else { "nav-link" },
                to: route.clone(),
                aria_label: "Switch to {label} page",
                aria_current: if is_active { "page" } else { "false" },
                "{label}"
                if is_active {
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
        ("Blog", Route::Blog {}, true),
        ("Projects", Route::Projects {}, false),
        ("Creative", Route::Creative {}, false),
        ("Contact", Route::Contact {}, true),
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
                        for (label , route , enabled) in links.iter() {
                            NavItemLink {
                                label: *label,
                                route: route.clone(),
                                enabled: *enabled,
                            }
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
    rsx! {
        button {
            id: "theme-toggle",
            class: "btn",
            r#type: "button",
            span { class: "sr-only theme-light-only", "Switch to dark theme" }
            span { class: "sr-only theme-dark-only", "Switch to light theme" }
            span { class: "theme-light-only", aria_hidden: "true", "Dark" }
            span { class: "theme-dark-only", aria_hidden: "true", "Light" }
        }
    }
}

#[component]
pub fn Header() -> Element {
    rsx! {
        NavBar {}
    }
}
