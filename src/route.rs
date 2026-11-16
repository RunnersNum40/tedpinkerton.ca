use crate::{layouts::HeaderFooter, views::*};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub(crate) enum Route {
    #[route("/badge/")]
    Badge {},
    #[layout(HeaderFooter)]
        #[route("/")]
        Home {},
        #[route("/blog/")]
        Blog {},
        #[route("/blog/:slug")]
        BlogPost { slug: String },
        #[route("/projects/")]
        Projects {},
        #[route("/creative/")]
        Creative {},
        #[route("/contact/")]
        Contact {},
        #[route("/resume/")]
        Resume {},
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}
