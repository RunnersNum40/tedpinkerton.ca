use crate::*;

#[component]
pub fn Page(
    id: String,
    name: Option<String>,
    description: String,
    body: Element,
    title: Option<String>,
    image: Option<ShareImage>,
    article: Option<ArticleMetadata>,
    #[props(default)] noindex: bool,
) -> Element {
    let title = title.or_else(|| name.clone()).unwrap_or_default();
    rsx! {
        PageMetadata { title, description, image, article, noindex }
        section { id: "{id}",
            div { class: "site-container",
                if let Some(name) = name {
                    h1 { "{name}" }
                }
                {body}
            }
        }
    }
}
