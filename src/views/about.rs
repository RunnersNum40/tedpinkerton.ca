use dioxus::prelude::*;

const ABOUT_CSS: Asset = asset!("/assets/styling/about.css");

#[component]
pub fn About() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: ABOUT_CSS }

        div {
            id: "about",
        }
    }
}
