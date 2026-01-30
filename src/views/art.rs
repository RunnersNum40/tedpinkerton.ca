use crate::*;

#[component]
pub fn Art() -> Element {
    rsx! {
        Page {
            id: "art",
            name: "Art",
            body: rsx! {
                p { "Work in progress :)" }
                p { "Imagine something cool." }
            },
        }
    }
}
