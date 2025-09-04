use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        section {
            id: "about", class: "section",
        }
    }
}

