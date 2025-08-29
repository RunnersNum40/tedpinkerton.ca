use chrono::prelude::*;
use dioxus::prelude::*;

const FOOTER_CSS: Asset = asset!("/assets/styling/footer.css");

#[component]
pub fn Footer() -> Element {
    let year = Local::now().year();
    rsx! {
        document::Link { rel: "stylesheet", href: FOOTER_CSS }
        footer { id: "footer", span { "© {year} Ted Pinkerton" } }
    }
}

