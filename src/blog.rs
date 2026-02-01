use chrono::NaiveDate;
use dioxus_typst::CompileOptions;
use include_dir::{Dir, include_dir};

static BLOG_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/content/blog");

pub fn all_slugs() -> Vec<String> {
    BLOG_DIR
        .dirs()
        .filter_map(|dir| dir.path().file_name()?.to_str().map(String::from))
        .collect()
}

pub fn get_meta(slug: &str) -> Option<String> {
    let file = BLOG_DIR.get_file(format!("{}/meta.toml", slug))?;
    Some(file.contents_utf8()?.to_string())
}

pub fn get_body(slug: &str) -> Option<String> {
    BLOG_DIR
        .get_file(format!("{}/body.typ", slug))
        .and_then(|f| f.contents_utf8())
        .map(String::from)
}

pub fn get_post_files(slug: &str) -> CompileOptions {
    tracing::info!("Loading blog post files for slug {}", slug);
    let mut options = CompileOptions::new();

    if let Some(dir) = BLOG_DIR.get_dir(slug) {
        tracing::debug!("Found blog post files dir {}", dir.path().display());
        for file in dir.files() {
            let filename = file
                .path()
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");

            tracing::debug!("Found blog post file: {}", filename);

            if filename.ends_with("meta.toml") || filename.ends_with("body.typ") {
                continue;
            }

            tracing::debug!("Adding blog post file to compile options: {}", filename);

            let path = format!("{}/{}", slug, filename);
            options = options.with_file(path, file.contents().to_vec());
        }
    }

    tracing::info!("Loaded blog post files for slug {}", slug);
    tracing::debug!("Compile options: {:?}", options);
    options
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

pub struct Meta {
    pub title: String,
    pub date: NaiveDate,
    pub summary: Option<String>,
    pub draft: bool,
}

pub fn parse_meta(content: &str) -> Option<Meta> {
    let raw: RawMeta = toml::from_str(content).ok()?;
    let date = NaiveDate::parse_from_str(&raw.date, "%Y-%m-%d").ok()?;
    Some(Meta {
        title: raw.title,
        date,
        summary: raw.summary,
        draft: raw.draft.unwrap_or(false),
    })
}

pub fn first_paragraph_summary(body: &str) -> String {
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
