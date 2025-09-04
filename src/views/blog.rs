use dioxus::prelude::*;

#[component]
pub fn Blog() -> Element {
    rsx! {
        section {
            id: "blog", class: "section",
        }
    }
}

