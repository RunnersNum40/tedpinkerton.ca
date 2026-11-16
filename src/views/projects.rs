use crate::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        Page {
            id: "projects",
            name: "Projects",
            body: rsx! {
                p { "Work in progress :)" }
            },
        }
    }
}
