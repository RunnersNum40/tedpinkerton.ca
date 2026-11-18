use crate::Route;
use dioxus::prelude::*;

const SITE_NAME: &str = "Ted Pinkerton";
const SITE_ORIGIN: &str = "https://tedpinkerton.ca";
const DEFAULT_SHARE_IMAGE: Asset = asset!("/assets/social.png");

#[derive(Clone, PartialEq)]
pub struct ShareImage {
    pub asset: Asset,
    pub alt: String,
    pub width: u32,
    pub height: u32,
}

impl Default for ShareImage {
    fn default() -> Self {
        Self {
            asset: DEFAULT_SHARE_IMAGE,
            alt: "tedpinkerton.ca".to_string(),
            width: 1200,
            height: 630,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ArticleMetadata {
    pub published: Option<chrono::NaiveDate>,
    pub authors: Vec<String>,
}

#[component]
pub fn PageMetadata(
    title: String,
    description: String,
    image: Option<ShareImage>,
    article: Option<ArticleMetadata>,
    #[props(default)] noindex: bool,
) -> Element {
    let route: Route = use_route();
    let document_title = format!(
        "tedpinkerton{}",
        route.to_string().trim_end_matches('/').to_lowercase()
    );
    let title = if title.is_empty() || title == SITE_NAME {
        SITE_NAME.to_string()
    } else {
        format!("{title}/{SITE_NAME}")
    };
    let canonical_url =
        (!noindex).then(|| format!("{SITE_ORIGIN}{}/", route.to_string().trim_end_matches('/')));
    let image = image.unwrap_or_default();
    let image_url = format!(
        "{SITE_ORIGIN}/{}",
        image.asset.to_string().trim_start_matches('/')
    );

    let robots = if noindex {
        "noindex,follow"
    } else {
        "index,follow"
    };
    let page_type = if article.is_some() {
        "article"
    } else {
        "website"
    };

    rsx! {
        document::Title { "{document_title}" }
        document::Meta { name: "description", content: description.clone() }
        document::Meta { name: "robots", content: robots }
        document::Meta { property: "og:title", content: title.clone() }
        document::Meta { property: "og:description", content: description.clone() }
        document::Meta { property: "og:site_name", content: SITE_NAME }
        document::Meta { property: "og:type", content: page_type }
        document::Meta { property: "og:image", content: image_url.clone() }
        document::Meta { property: "og:image:alt", content: image.alt.clone() }
        document::Meta { property: "og:image:width", content: image.width.to_string() }
        document::Meta { property: "og:image:height", content: image.height.to_string() }
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: title }
        document::Meta { name: "twitter:description", content: description }
        document::Meta { name: "twitter:image", content: image_url }
        document::Meta { name: "twitter:image:alt", content: image.alt }
        if let Some(url) = canonical_url {
            document::Link { rel: "canonical", href: url.clone() }
            document::Meta { property: "og:url", content: url }
        }
        if let Some(article) = article {
            if let Some(date) = article.published {
                document::Meta { property: "article:published_time", content: date.to_string() }
            }
            if !article.authors.is_empty() {
                document::Meta { name: "author", content: article.authors.join(", ") }
            }
        }
    }
}
