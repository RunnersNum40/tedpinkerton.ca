use crate::Theme;
use dioxus::prelude::*;

const ICON_LIGHT: Asset = asset!("/assets/icons/icon_light.png");
const ICON_DARK: Asset = asset!("/assets/icons/icon_dark.png");

#[component]
pub fn Logo() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let src = match theme() {
        Theme::Light => ICON_LIGHT,
        Theme::Dark => ICON_DARK,
    };

    rsx! {
        img { class: "logo relative top-[0.2rem]", src: "{src}", alt: "", aria_hidden: "true" }
    }
}
