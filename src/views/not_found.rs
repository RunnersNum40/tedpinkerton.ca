use crate::components::Page;
use dioxus::prelude::*;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        Page {
            id: "not-found",
            name: "Not found",
            description: "This page could not be found on Ted Pinkerton's website.",
            noindex: true,
            body: rsx! { p { "Page not found." } },
        }
    }
}
