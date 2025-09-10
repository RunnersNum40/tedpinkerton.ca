use crate::*;
use dioxus::prelude::*;

#[component]
pub fn Page(id: String, name: String, body: Element) -> Element {
    let path: Route = use_route();

    let rendered_path = match path {
        Route::Home {} => "".to_string(),
        _ => {
            let trimmed = path.to_string().trim_end_matches('/').to_lowercase();

            trimmed
        }
    };

    rsx! {
        document::Title { "tedpinkerton{rendered_path}" }
        section {
            id: "{id}",
            div { class: "site-container",
                h1 { "{name}" }
                {body}
            }
        }
    }
}
