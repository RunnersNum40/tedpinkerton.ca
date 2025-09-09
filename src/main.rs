use dioxus::prelude::*;
use dioxus_sdk::{storage::*, theme::*};

mod components;
mod theme;
mod views;

use theme::*;
use views::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(HeaderFooter)]
        #[route("/")]
        Home {},
        #[route("/blog/")]
        Blog {},
        #[route("/projects/")]
        Projects {},
        #[route("/art/")]
        Art {},
        #[route("/about/")]
        About {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const TOKENS_CSS: Asset = asset!("/assets/css/tokens.css");
const BASE_CSS: Asset = asset!("/assets/css/base.css");
const LAYOUT_CSS: Asset = asset!("/assets/css/layout.css");
const NAV_CSS: Asset = asset!("/assets/css/nav.css");
const FOOTER_CSS: Asset = asset!("/assets/css/footer.css");
const COMPONENTS_CSS: Asset = asset!("/assets/css/components.css");

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
        let system_theme = system_theme.clone();
        let stored_theme = stored_theme.clone();
        move || resolve_theme(system_theme(), stored_theme())
    });

    {
        let system_theme = system_theme.clone();
        let stored_theme = stored_theme.clone();
        let mut theme = theme.clone();
        use_effect(move || {
            theme.set(resolve_theme(system_theme(), stored_theme()));
        });
    }

    use_context_provider(|| stored_theme);
    use_context_provider(|| theme);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        document::Link { rel: "stylesheet", href: TOKENS_CSS }
        document::Link { rel: "stylesheet", href: BASE_CSS }
        document::Link { rel: "stylesheet", href: LAYOUT_CSS }
        document::Link { rel: "stylesheet", href: NAV_CSS }
        document::Link { rel: "stylesheet", href: FOOTER_CSS }
        document::Link { rel: "stylesheet", href: COMPONENTS_CSS }

        div { id: "app", "data-theme": "{theme()}",
            Router::<Route> {}
        }
    }
}
