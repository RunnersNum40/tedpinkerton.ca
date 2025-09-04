use dioxus::prelude::*;

#[component]
pub fn Blog() -> Element {
    rsx! {
        section { id: "blog",
            div { class: "container",
                h1 { "Blog" }
                p { class: "muted", "Posts and notes on software engineering." }
            }
        }
    }
}

