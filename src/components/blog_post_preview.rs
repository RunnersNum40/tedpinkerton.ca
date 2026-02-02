use crate::*;
use chrono::NaiveDate;
use dioxus::prelude::*;

#[component]
pub fn BlogPostPreview(
    title: String,
    date: Option<NaiveDate>,
    summary: String,
    link: String,
) -> Element {
    let (iso, human) = if let Some(date) = date {
        (date.to_string(), date.format("%B %d, %Y").to_string())
    } else {
        ("".to_string(), "Unknown date".to_string())
    };

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
