use chrono::prelude::*;

use crate::*;

#[component]
pub fn Footer() -> Element {
    let year = Local::now().year();

    rsx! {
        footer { id: "footer",
            div { class: "footer-inner",
                span { "© {year} Theodore Pinkerton" }
                SkuleWebring {}
                nav { class: "footer-links",
                    Link {
                        class: "footer-link",
                        "aria-label": "Site GitHub Repository Link",
                        to: "https://github.com/RunnersNum40/tedpinkerton.ca",
                        "Site Source"
                    }
                }
            }
        }
    }
}
