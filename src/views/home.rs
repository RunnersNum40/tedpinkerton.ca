use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        section {
            id: "home", class: "home",
        }
    }
}
