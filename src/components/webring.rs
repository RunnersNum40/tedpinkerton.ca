use dioxus::prelude::*;

#[component]
pub fn SkuleWebring() -> Element {
    let SITE_URL = "https://tedpinkerton.ca";

    rsx! {
        nav { class: "webring", aria_label: "Skule WebRing",
            Link {
                class: "webring-arrow",
                to: "https://webring.skule.ca/#{SITE_URL}?nav=prev",
                aria_label: "Previous site in the Skule WebRing",
                span { aria_hidden: "true", "←" }
            }
            Link {
                class: "webring-center",
                to: "https://webring.skule.ca/#{SITE_URL}",
                rel: "noopener noreferrer",
                img {
                    class: "theme-light-only",
                    src: "https://webring.skule.ca/img/icon.svg",
                    alt: "Skule WebRing",
                }
                img {
                    class: "theme-dark-only",
                    src: "https://webring.skule.ca/img/icon-dark.svg",
                    alt: "Skule WebRing",
                }
            }
            Link {
                class: "webring-arrow",
                to: "https://webring.skule.ca/#{SITE_URL}?nav=next",
                aria_label: "Next site in the Skule WebRing",
                span { aria_hidden: "true", "→" }
            }
        }
    }
}
