use crate::*;

#[component]
pub fn Creative() -> Element {
    rsx! {
        Page {
            id: "creative",
            name: "Creative",
            body: rsx! {
                p { "Work in progress :)" }
                p { "Imagine something cool." }
            },
        }
    }
}
