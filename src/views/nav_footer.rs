use crate::{Route, components::{Navbar, Footer}};
use dioxus::prelude::*;

#[component]
pub fn NavFooter() -> Element {
    rsx! {
        div { id: "layout",
            Navbar {}
            div { id: "main", Outlet::<Route> {} }
            Footer {}
        }
    }
}
