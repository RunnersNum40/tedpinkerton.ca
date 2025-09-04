use crate::{Route, StorageTheme, Theme};
use dioxus::prelude::*;

#[component]
fn NavBar() -> Element {
    let links = [
        ("Home", Route::Home {}),
        ("Blog", Route::Blog {}),
        ("Projects", Route::Projects {}),
        ("Art", Route::Art {}),
        ("About", Route::About {}),
    ];

    rsx! {
        nav { id: "navbar",
            a { href: "#main", class: "sr-only", "Skip to content" }
            div { class: "nav-inner",
                div { class: "nav-home",
                    Link {
                        class: "nav-home-link",
                        to: Route::Home {},
                        "tedpinkerton.ca"
                    }
                }
                div { class: "nav-links",
                    for (label, route) in links.iter() {
                        Link {
                            class: "nav-link",
                            to: route.clone(),
                            aria_label: "Switch to {label} page",
                            "{label}"
                        }
                    }
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
    let theme = use_context::<Signal<Theme>>();
    let mut stored_setting = use_context::<Signal<StorageTheme>>();

    let (aria_label, pressed) = match theme() {
        Theme::Light => ("Switch to dark theme", false),
        Theme::Dark => ("Switch to light theme", true),
    };

    let label = match theme() {
        Theme::Light => "Dark",
        Theme::Dark => "Light",
    };

    rsx! {
        NavBar {}
        ThemeToggle {}
    }
}
