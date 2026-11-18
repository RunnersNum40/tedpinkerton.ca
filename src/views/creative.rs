use crate::*;

#[component]
pub fn Creative() -> Element {
    rsx! {
        Page {
            id: "creative",
            name: "Creative",
            description: "A space for Ted Pinkerton's creative work. More coming soon.",
            body: rsx! {
                p { "Work in progress :)" }
                p { "Imagine something cool." }
            },
        }
    }
}
