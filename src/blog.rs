use chrono::NaiveDate;
use dioxus_typst::{CompileOptions, DocumentMetadata, extract_metadata};
use include_dir::{Dir, include_dir};

static BLOG_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/content/blog");

pub fn all_slugs() -> Vec<String> {
    BLOG_DIR
        .dirs()
        .filter_map(|dir| dir.path().file_name()?.to_str().map(String::from))
        .collect()
}

pub fn get_body(slug: &str) -> Option<String> {
    BLOG_DIR
        .get_file(format!("{}/body.typ", slug))
        .and_then(|f| f.contents_utf8())
        .map(String::from)
}

pub fn get_post_files(slug: &str) -> CompileOptions {
    let mut options = CompileOptions::new();

    if let Some(dir) = BLOG_DIR.get_dir(slug) {
        for file in dir.files() {
            let filename = file
                .path()
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");

            if filename.ends_with("body.typ") {
                continue;
            }

            let path = format!("{}/{}", slug, filename);
            options = options.with_file(filename, file.contents().to_vec());
        }
    }

    options
}

pub fn get_post_meta(slug: &str) -> Option<DocumentMetadata> {
    let source = get_body(slug)?;
    let options = get_post_files(slug);
    extract_metadata(&source, &options).ok()
}

pub fn is_draft(meta: &DocumentMetadata) -> bool {
    meta.keywords
        .iter()
        .any(|k| k.eq_ignore_ascii_case("draft"))
}

pub fn first_header_title(body: &str) -> String {
    let line = body
        .lines()
        .map(str::trim)
        .find(|l| l.starts_with('='))
        .unwrap_or("");
    line.trim_start_matches('=')
        .trim_start_matches('#')
        .trim()
        .to_string()
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

pub fn all_blog_previews() -> Vec<(String, Option<NaiveDate>, String, String)> {
    let mut items = vec![];
    let slugs = all_slugs();

    for slug in slugs {
        if let Some(meta) = get_post_meta(&slug)
            && (!is_draft(&meta) || cfg!(debug_assertions))
        {
            let title = meta.title.clone().unwrap_or_else(|| {
                get_body(&slug)
                    .map(|b| first_header_title(&b))
                    .unwrap_or_else(|| slug.clone())
            });
            let summary = meta.description.unwrap_or_else(|| {
                get_body(&slug)
                    .map(|b| first_paragraph_summary(&b))
                    .unwrap_or_default()
            });
            let link = format!("/blog/{}", slug);
            items.push((title, meta.date, summary, link));
        }
    }

    items.sort_by(|a, b| b.1.cmp(&a.1));

    items
}
