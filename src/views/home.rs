use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        section {
            id: "home",
            div { class: "site-container",
                h1 { "Welcome to my personal website!" }
                p { "Feel free to explore my blog, projects, and artwork using the navigation links above." }
            }
        }
    }
}
