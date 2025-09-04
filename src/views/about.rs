use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        section {
            id: "about",
            div { class: "container",
                h1 { "About" }
                p { class: "muted", "A little about me." }
            }
        }
    }
}
