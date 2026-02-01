use crate::*;
use crate::blog::{get_body, get_meta, get_post_files, parse_meta};
use dioxus::prelude::*;
use dioxus_typst::Typst;

#[component]
pub fn BlogPost(slug: String) -> Element {
    if let (Some(meta_str), Some(body)) = (get_meta(&slug), get_body(&slug))
        && let Some(meta) = parse_meta(&meta_str)
    {
        let iso = meta.date.to_string();
        let human = meta.date.format("%B %d, %Y").to_string();
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
