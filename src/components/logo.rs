use dioxus::prelude::*;

const ICON_LIGHT: Asset = asset!("/assets/icons/icon_light.png");
const ICON_DARK: Asset = asset!("/assets/icons/icon_dark.png");

#[component]
pub fn Logo() -> Element {
    rsx! {
        img {
            class: "logo relative top-02 theme-light-only",
            src: ICON_LIGHT,
            alt: "",
            aria_hidden: "true",
        }
        img {
            class: "logo relative top-02 theme-dark-only",
            src: ICON_DARK,
            alt: "",
            aria_hidden: "true",
        }
    }
}
