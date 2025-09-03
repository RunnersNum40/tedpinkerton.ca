use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Blog {}, "Blog" }
            Link { to: Route::Projects {}, "Projects" }
            Link { to: Route::Art {}, "Art" }
            Link { to: Route::About {}, "About" }
        }
    }
}
