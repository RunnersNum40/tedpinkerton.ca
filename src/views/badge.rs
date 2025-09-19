use dioxus::prelude::*;

const BADGE_IMAGE: Asset = asset!("/assets/icons/badge.png");

#[component]
pub fn Badge() -> Element {
    rsx! {
        img { class: "h-6 w-6", src: "{BADGE_IMAGE}", alt: "Site Badge", aria_hidden: "true" }
    }
}
