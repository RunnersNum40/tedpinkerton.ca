use crate::*;
use dioxus::prelude::*;

#[component]
pub fn Blog() -> Element {
    rsx! {
        Page {
            id: "blog",
            name: "Blog",
            body: rsx! {
                p { "Work in progress :)" }
            }
        }
    }
}
