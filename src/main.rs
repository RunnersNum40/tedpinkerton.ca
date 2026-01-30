use dioxus::prelude::*;
use dioxus_sdk_storage::*;
use dioxus_sdk_window::theme::{
    Theme as SystemTheme, ThemeError as SystemThemeError, use_system_theme,
};

mod components;
mod theme;
mod views;

use components::*;
use theme::*;
use views::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/badge/")]
    Badge {},
    #[layout(HeaderFooter)]
        #[route("/")]
        Home {},
        #[route("/blog/")]
        Blog {},
        #[route("/blog/:slug")]
        BlogPost { slug: String },
        #[route("/projects/")]
        Projects {},
        #[route("/art/")]
        Art {},
        #[route("/contact/")]
        Contact {},
        #[route("/resume/")]
        Resume {},
}

const FAVICON_LIGHT: Asset = asset!("/assets/icons/favicon_light.ico");
const FAVICON_DARK: Asset = asset!("/assets/icons/favicon_dark.ico");

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const MAIN_CSS: Asset = asset!("/assets/scss/main.scss");

fn main() {
    dioxus::launch(App);
}

type StorageTheme = Option<Theme>;

fn resolve_theme(system: Result<SystemTheme, SystemThemeError>, setting: StorageTheme) -> Theme {
    match setting {
        Some(Theme::Light) => Theme::Light,
        Some(Theme::Dark) => Theme::Dark,
        _ => match system {
            Ok(system) => system.into(),
            Err(_) => Theme::Light,
        },
    }
}

#[component]
fn App() -> Element {
    let system_theme = use_system_theme();
    let stored_theme = use_persistent("theme", StorageTheme::default);

    let theme = use_signal({
        move || resolve_theme(system_theme(), stored_theme())
    });

    {
        let mut theme = theme;
        use_effect(move || {
            theme.set(resolve_theme(system_theme(), stored_theme()));
        });
    }

    use_context_provider(|| stored_theme);
    use_context_provider(|| theme);

    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Title { "Ted Pinkerton" }

        document::Link {
            rel: "icon",
            href: FAVICON_LIGHT,
            media: "(prefers-color-scheme: light)",
        }
        document::Link {
            rel: "icon",
            href: FAVICON_DARK,
            media: "(prefers-color-scheme: dark)",
        }

        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { id: "app", "data-theme": "{theme()}", Router::<Route> {} }
    }
}
