use chrono::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    let year = Local::now().year();

    rsx! {
        footer { id: "footer",
            div { class: "footer-inner",
                span { "© {year} Ted Pinkerton" }
                nav {
                    class: "footer-links",
                    Link {
                        class: "footer-link",
                        "aria-label": "Site GitHub Repository Link",
                        to: "https://github.com/RunnersNum40/site",
                        "Site Source"
                    }
                    Link {
                        class: "footer-link",
                        "aria-label": "Skule Webring Link",
                        to: "https://webring.skule.ca",
                        "Skule webring"
                    }
                }
            }
        }
    }
}
