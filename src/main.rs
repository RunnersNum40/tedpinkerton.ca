use dioxus::prelude::*;

mod blog;
mod components;
mod layouts;
#[expect(unreachable_code)]
mod route;
mod theme;
mod views;

use components::*;
use route::Route;
use theme::*;

const FAVICON_LIGHT: Asset = asset!("/assets/icons/favicon_light.ico");
const FAVICON_DARK: Asset = asset!("/assets/icons/favicon_dark.ico");

const MAIN_CSS: Asset = asset!("/assets/scss/main.scss");

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            ServeConfig::builder().incremental(
                dioxus::server::IncrementalRendererConfig::new()
                    .static_dir(
                        std::env::current_exe()
                            .expect("Cannot locate server executable")
                            .parent()
                            .expect("Server executable has no parent directory")
                            .join("public"),
                    )
                    .clear_cache(false),
            )
        })
        .launch(App);
}

#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    let mut routes: Vec<String> = Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect();
    for slug in blog::all_slugs() {
        let metadata = blog::get_post_meta(&slug)
            .ok_or_else(|| ServerFnError::new(format!("Cannot compile blog post {slug}")))?;
        if !blog::is_draft(&metadata) {
            routes.push(Route::BlogPost { slug }.to_string());
        }
    }
    routes.sort();
    routes.push("/404".to_string());
    Ok(routes)
}

#[component]
fn App() -> Element {
    let theme = use_theme();
    use_context_provider(|| theme);

    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Title { "Ted Pinkerton" }

        document::Link {
            rel: "icon",
            href: FAVICON_LIGHT,
            media: "(prefers-color-scheme: light)",
        }
        document::Link {
            rel: "icon",
            href: FAVICON_DARK,
            media: "(prefers-color-scheme: dark)",
        }

        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { id: "app", "data-theme": "{theme()}", Router::<Route> {} }
    }
}
