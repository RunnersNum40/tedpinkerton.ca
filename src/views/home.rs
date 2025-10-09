use crate::*;
use dioxus::prelude::*;

#[component]
fn HomeContent() -> Element {
    rsx! {
        div {
            p {
                "Hello! Welcome to my corner of the matrix. "
                "This is a work in progress, I plan to host some projects but for now it's just this small section. "
            }
            // p {
            //     "I'm Ted Pinkerton, an open source robotics and software enthusiast, a procedural artist, and a bit of an optimist. "
            //     "I have a Bachelor's of Applied Science in Engineering Science from the University of Toronto with a major in Machine Intelligence and a minor in Robotics. "
            //     "I plan to use that to get robots in space but it's slow going at the moment. "
            //     "In general I just get a kick out of computers doing interesting things. "
            // }
        }
    }
}

#[component]
pub fn Home() -> Element {
    rsx! {
        Page{
            id: "home",
            name: "Ted Pinkerton",
            body: rsx! {
                HomeContent {}
            }
        }
    }
}
