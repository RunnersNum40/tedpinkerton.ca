use dioxus::prelude::*;

#[component]
pub fn Blog() -> Element {
    rsx! {
        section { id: "blog",
            div { class: "site-container",
                h1 { "Blog" }
                p { "Posts and notes." }
            }
        }
    }
}
