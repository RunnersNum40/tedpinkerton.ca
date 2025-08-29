use dioxus::prelude::*;

const PROJECTS_CSS: Asset = asset!("/assets/styling/projects.css");

#[component]
pub fn Projects() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: PROJECTS_CSS }

        div {
            id: "projects",
        }
    }
}
