use crate::{
    components::{Footer, Header},
    Route,
};
use dioxus::prelude::*;

#[component]
pub fn HeaderFooter() -> Element {
    rsx! {
        Header {}
        main { id: "main", Outlet::<Route> {} }
        Footer {}
    }
}
