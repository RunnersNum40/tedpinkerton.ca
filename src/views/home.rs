use crate::*;
use dioxus::prelude::*;

#[component]
fn HomeContent() -> Element {
    rsx! {
        div {
            p {
                "Hello and welcome to my corner of the matrix. "
                "Under construction, but feel free to look around!"
            }
        }
    }
}

#[component]
pub fn Home() -> Element {
    rsx! {
        Page {
            id: "home",
            name: "Ted Pinkerton",
            body: rsx! {
                HomeContent {}
            },
        }
    }
}
