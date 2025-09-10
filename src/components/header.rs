use crate::{Route, StorageTheme, Theme};
use dioxus::prelude::*;

const ICON_LIGHT: Asset = asset!("/assets/icon_light.png");
const ICON_DARK: Asset = asset!("/assets/icon_dark.png");

#[component]
fn SiteLogo() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let src = match theme() {
        Theme::Light => ICON_LIGHT,
        Theme::Dark => ICON_DARK,
    };

    rsx! {
        img { class: "logo relative top-[0.2rem]", src: "{src}", alt: "", aria_hidden: "true" }
    }
}

#[component]
fn NavBar() -> Element {
    let links = [
        ("Home", Route::Home {}),
        ("Blog", Route::Blog {}),
        ("Projects", Route::Projects {}),
        ("Art", Route::Art {}),
        ("Contact", Route::Contact {}),
        ("Resume", Route::Resume {}),
    ];

    let current: Route = use_route();

    rsx! {
        nav { id: "navbar",
            a { href: "#main", class: "sr-only", "Skip to content" }
            div { class: "nav-inner",
                div { class: "nav-home",
                    Link { class: "nav-home-link", to: Route::Home {},
                        SiteLogo {}
                        span { "tedpinkerton.ca" }
                    }
                }
                div { class: "nav-right",
                    div { class: "nav-links",
                        for (label, route) in links.iter() {
                            Link {
                                class: "nav-link",
                                active_class: "active",
                                to: route.clone(),
                                aria_label: "Switch to {label} page",
                                "{label}"
                                if current == *route {
                                    span { class: "sr-only", " (current page)" }
                                }
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
