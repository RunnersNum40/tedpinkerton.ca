use crate::blog::{
    first_header_title, first_paragraph_summary, get_body, get_post_files, get_post_meta, is_draft,
};
use crate::*;
use dioxus_typst::Typst;

#[component]
pub fn BlogPost(slug: String) -> Element {
    if let (Some(meta), Some(body)) = (get_post_meta(&slug), get_body(&slug)) {
        let (iso, human) = if let Some(date) = meta.date {
            (date.to_string(), date.format("%B %d, %Y").to_string())
        } else {
            ("".to_string(), "Unknown date".to_string())
        };
        let options = get_post_files(&slug);
        let title = meta
            .title
            .clone()
            .unwrap_or_else(|| first_header_title(&body));
        let description = meta
            .description
            .clone()
            .unwrap_or_else(|| first_paragraph_summary(&body));
        let noindex = is_draft(&meta);
        return rsx! {
            Page {
                id: "blog-post",
                title,
                description,
                noindex,
                article: ArticleMetadata { published: meta.date, authors: meta.authors },
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
            description: "This blog post could not be found on Ted Pinkerton's website.",
            noindex: true,
            body: rsx! {
                p { "Post not found." }
            },
        }
    }
}
