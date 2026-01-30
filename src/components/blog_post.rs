use crate::*;
use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_typst::Typst;
use include_dir::{Dir, include_dir};

static BLOG_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/content/blog");

fn all_slugs() -> Vec<String> {
    BLOG_DIR
        .dirs()
        .filter_map(|dir| dir.path().file_name()?.to_str().map(String::from))
        .collect()
}

fn get_meta(slug: &str) -> Option<String> {
    let file = BLOG_DIR.get_file(format!("{}/meta.toml", slug))?;
    Some(file.contents_utf8()?.to_string())
}

fn get_body(slug: &str) -> Option<String> {
    BLOG_DIR
        .get_file(format!("{}/body.typ", slug))
        .and_then(|f| f.contents_utf8())
        .map(String::from)
}

#[derive(serde::Deserialize)]
struct RawMeta {
    title: String,
    date: String,
    #[serde(default)]
    summary: Option<String>,
    #[serde(default)]
    draft: Option<bool>,
}

struct Meta {
    title: String,
    date: NaiveDate,
    summary: Option<String>,
    draft: bool,
}

fn parse_meta(content: &str) -> Option<Meta> {
    let raw: RawMeta = toml::from_str(content).ok()?;
    let date = NaiveDate::parse_from_str(&raw.date, "%Y-%m-%d").ok()?;
    Some(Meta {
        title: raw.title,
        date,
        summary: raw.summary,
        draft: raw.draft.unwrap_or(false),
    })
}

fn first_paragraph_summary(body: &str) -> String {
    let line = body
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty() && !l.starts_with('=') && !l.starts_with('#'))
        .unwrap_or("");
    let mut out = line.to_string();
    if out.len() > 200 {
        out.truncate(200);
        out.push('…');
    }
    out
}

#[component]
pub fn BlogPost(slug: String) -> Element {
    if let (Some(meta_str), Some(body)) = (get_meta(&slug), get_body(&slug))
        && let Some(meta) = parse_meta(&meta_str)
    {
        let iso = meta.date.to_string();
        let human = meta.date.format("%B %d, %Y").to_string();
        return rsx! {
            Page {
                id: "blog-post",
                body: rsx! {
                    article { class: "blog-post",
                        p { class: "muted small",
                            time { datetime: "{iso}", "on {human}" }
                        }
                        Typst { source: body, class: "typst-content".to_string() }
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

pub fn all_blog_previews() -> Vec<(String, NaiveDate, String, String)> {
    let mut items = vec![];
    let slugs = all_slugs();

    for slug in slugs {
        if let Some(meta_str) = get_meta(&slug)
            && let Some(meta) = parse_meta(&meta_str)
            && (!meta.draft || cfg!(debug_assertions))
        {
            let summary = meta.summary.unwrap_or_else(|| {
                get_body(&slug)
                    .map(|b| first_paragraph_summary(&b))
                    .unwrap_or_default()
            });
            let link = format!("/blog/{}", slug);
            items.push((meta.title, meta.date, summary, link));
        }
    }

    items.sort_by(|a, b| b.1.cmp(&a.1));

    items
}

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
