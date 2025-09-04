use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        section {
            id: "projects",
            div { class: "container",
                h1 { "Projects" }
                p { class: "muted", "A selection of my software projects." }
            }
        }
    }
}
