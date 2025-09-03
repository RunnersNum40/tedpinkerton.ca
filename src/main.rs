use dioxus::prelude::*;

mod components;
mod views;

use views::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(NavFooter)]
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
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            id: "app",
            Router::<Route> {}
        }
    }
}
