use crate::components::Page;
use dioxus::prelude::*;

#[component]
fn AboutContent() -> Element {
    rsx! {
        article {
            nav { class: "social-links", aria_label: "Social links",
                ul { class: "social-list",
                    li { class: "social-item",
                        span { class: "label", "GitHub:" }
                        Link {
                            class: "social-link",
                            aria_label: "Open GitHub profile",
                            to: "https://github.com/RunnersNum40",
                            "github.com/RunnersNum40"
                        }
                    }
                    li { class: "social-item",
                        span { class: "label", "LinkedIn:" }
                        Link {
                            class: "social-link",
                            aria_label: "Open LinkedIn profile",
                            to: "https://www.linkedin.com/in/ted-pinkerton/",
                            "linkedin.com/in/ted-pinkerton"
                        }
                    }
                    li { class: "social-item",
                        span { class: "label", "Email:" }
                        Link {
                            class: "social-link",
                            aria_label: "Send email to p_t@fastmail.net",
                            to: "mailto:ted@tedpinkerton.ca",
                            "ted@tedpinkerton.ca"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Contact() -> Element {
    rsx! {
        Page { id: "about", name: "Contact", body: AboutContent() }
    }
}
