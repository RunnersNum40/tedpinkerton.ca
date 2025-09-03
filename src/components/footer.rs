use chrono::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    let year = Local::now().year();
    rsx! {
        footer { id: "footer", span { "© {year} Ted Pinkerton" } }
    }
}
