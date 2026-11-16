use crate::components::Page;
use dioxus::prelude::*;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        Page {
            id: "not-found",
            name: "Not found",
            body: rsx! { p { "Page not found." } },
        }
    }
}
