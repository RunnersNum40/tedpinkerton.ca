use crate::*;
use dioxus::prelude::*;

#[component]
pub fn Blog() -> Element {
    let previews = all_blog_previews();

    rsx! {
        Page {
            id: "blog",
            body: rsx! {
                if previews.is_empty() {
                    p { "No posts yet." }
                } else {
                    ul { class: "blog-list",
                        for (title, date, summary, link) in previews {
                            li { class: "blog-item", key: "{link}",
                                BlogPostPreview { title, date, summary, link: link.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}
