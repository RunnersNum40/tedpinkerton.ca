use crate::*;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        Page {
            id: "projects",
            name: "Projects",
            body: rsx! {
                p { "A selection of projects." }
            },
        }
    }
}
