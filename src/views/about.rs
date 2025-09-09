use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        section {
            id: "about",
            div { class: "site-container",
                h1 { "About" }
                p { "A little about me." }
            }
        }
    }
}
