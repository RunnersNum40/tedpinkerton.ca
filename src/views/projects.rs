use crate::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        Page {
            id: "projects",
            name: "Projects",
            description: "Projects by Ted Pinkerton. This page is a work in progress.",
            body: rsx! {
                p { "Work in progress :)" }
            },
        }
    }
}
