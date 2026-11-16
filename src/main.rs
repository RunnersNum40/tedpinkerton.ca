use dioxus::prelude::*;

mod blog;
mod components;
mod layouts;
#[expect(unreachable_code)]
mod route;
mod theme;
mod views;

use components::*;
use route::Route;
use theme::*;

const FAVICON_LIGHT: Asset = asset!("/assets/icons/favicon_light.ico");
const FAVICON_DARK: Asset = asset!("/assets/icons/favicon_dark.ico");

const MAIN_CSS: Asset = asset!("/assets/scss/main.scss");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let theme = use_theme();
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

        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { id: "app", "data-theme": "{theme()}", Router::<Route> {} }
    }
}
