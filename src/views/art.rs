use dioxus::prelude::*;

const ART_CSS: Asset = asset!("/assets/styling/art.css");

#[component]
pub fn Art() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: ART_CSS }

        div {
            id: "art",
        }
    }
}
