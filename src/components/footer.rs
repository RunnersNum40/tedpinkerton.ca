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
                    a { href: "https://webring.skule.ca", "Skule webring" }
                }
            }
        }
    }
}
