use dioxus::prelude::*;

#[component]
pub fn Art() -> Element {
    rsx! {
        section {
            id: "art",
            div { class: "container",
                h1 { "Art" }
                p { class: "muted", "A collection of my artwork." }
            }
        }
    }
}
