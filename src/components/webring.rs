use crate::theme::Theme;
use dioxus::prelude::*;

#[component]
pub fn SkuleWebring() -> Element {
    let SITE_URL = "https://tedpinkerton.ca";

    let theme = use_context::<Signal<Theme>>();
    let icon = match theme() {
        Theme::Light => "https://webring.skule.ca/img/icon.svg",
        Theme::Dark => "https://webring.skule.ca/img/icon-dark.svg",
    };

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
                img { src: "{icon}", alt: "Skule WebRing" }
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
