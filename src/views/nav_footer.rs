use crate::{
    components::{Footer, Navbar},
    Route,
};
use dioxus::prelude::*;

#[component]
pub fn NavFooter() -> Element {
    rsx! {
        div { id: "layout",
            Navbar {}
            main { id: "main", role: "main",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}

