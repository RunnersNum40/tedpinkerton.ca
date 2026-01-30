use crate::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        Page {
            id: "projects",
            name: "Projects",
            body: rsx! {
                p { "Work in progress :)" }
                p {
                    "Check "
                    Link { to: Route::Resume {}, "resume" }
                    " for more details."
                }
            },
        }
    }
}
