use crate::*;
use chrono::NaiveDate;
use dioxus::prelude::*;
use include_dir::{Dir, DirEntry, include_dir};

static BLOG_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/content/blog");

fn all_slugs() -> Vec<String> {
    BLOG_DIR
        .find("**/*.md")
        .map(|entries| {
            entries
                .filter_map(|e| match e {
                    DirEntry::File(f) => f
                        .path()
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_string()),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn get_file(slug: &str) -> Option<String> {
    let file = BLOG_DIR.get_file(&format!("{}.md", slug))?;
    Some(file.contents_utf8()?.to_string())
}

fn split_front_matter(content: &str) -> Option<(&str, &str)> {
    let mut parts = content.splitn(3, "---");
    let _leading = parts.next()?;
    let fm = parts.next()?;
    let body = parts.next()?;
    Some((fm, body))
}

#[derive(serde::Deserialize)]
struct RawFrontMatter {
    title: String,
    date: String,
    #[serde(default)]
    summary: Option<String>,
    draft: Option<bool>,
}

struct Meta {
    title: String,
    date: NaiveDate,
    summary: Option<String>,
    draft: Option<bool>,
}

fn parse_front_matter(fm: &str) -> Option<Meta> {
    let raw: RawFrontMatter = serde_yaml::from_str(fm).ok()?;
    let date = NaiveDate::parse_from_str(&raw.date, "%Y-%m-%d").ok()?;
    Some(Meta {
        title: raw.title,
        date,
        summary: raw.summary,
        draft: raw.draft,
    })
}

fn first_paragraph_summary(body: &str) -> String {
    let line = body
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with('>'))
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
    if let Some(content) = get_file(&slug) {
        if let Some((fm_raw, body)) = split_front_matter(&content) {
            if let Some(meta) = parse_front_matter(fm_raw) {
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
                                Markdown { content: body.to_string() }
                            }
                        },
                    }
                };
            }
        }
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

    for slug in all_slugs() {
        if let Some(content) = get_file(&slug) {
            if let Some((fm_raw, body)) = split_front_matter(&content) {
                if let Some(meta) = parse_front_matter(fm_raw) {
                    if !meta.draft.unwrap_or(false) || cfg!(debug_assertions) {
                        let summary = meta
                            .summary
                            .unwrap_or_else(|| first_paragraph_summary(body));
                        let link = format!("/blog/{}", slug);
                        items.push((meta.title, meta.date, summary, link));
                    }
                }
            }
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
