use crate::blog::{get_body, get_post_files, get_post_meta};
use crate::*;
use dioxus::prelude::*;
use dioxus_typst::{DocumentMetadata, Typst};

#[component]
pub fn BlogPost(slug: String) -> Element {
    if let (Some(meta), Some(body)) = (get_post_meta(&slug), get_body(&slug)) {
        let (iso, human) = if let Some(date) = meta.date {
            (date.to_string(), date.format("%B %d, %Y").to_string())
        } else {
            ("".to_string(), "Unknown date".to_string())
        };
        let options = get_post_files(&slug);
        return rsx! {
            Page {
                id: "blog-post",
                body: rsx! {
                    article { class: "blog-post",
                        p { class: "muted small",
                            time { datetime: "{iso}", "on {human}" }
                        }
                        Typst { source: body, options: options, class: "typst-content".to_string() }
                    }
                },
            }
        };
    }

    rsx! {
        Page {
            id: "blog-post",
            name: "Not found",
            body: rsx! {
                p { "Post not found." }
            },
        }
    }
}
