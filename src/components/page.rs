use dioxus::prelude::*;

#[component]
pub fn Page(id: String, name: String, body: Element) -> Element {
    rsx! {
        section {
            id: "{id}",
            div { class: "site-container",
                h1 { "{name}" }
                {body}
            }
        }
    }
}
