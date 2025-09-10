use crate::*;
use dioxus::prelude::*;

#[component]
pub fn Art() -> Element {
    rsx! {
        Page {
            id: "art",
            name: "Art",
            body: rsx! {
                p { "A collection of my artwork." }
            }
        }
    }
}
