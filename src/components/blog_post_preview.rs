use crate::*;
use chrono::NaiveDate;
use dioxus::prelude::*;

pub use crate::blog::all_blog_previews;

#[component]
pub fn BlogPostPreview(title: String, date: NaiveDate, summary: String, link: String) -> Element {
    let iso = date.to_string();
    let human = date.format("%B %d, %Y").to_string();

    rsx! {
        Link { to: link.clone(),
            article { class: "blog-card", key: "{link}",
                header { class: "blog-card-head",
                    p {
                        time { datetime: "{iso}", "on {human}" }
                    }
                    h1 { "{title}" }
                }
                p { class: "blog-card-summary", "{summary}" }
            }
        }
    }
}
