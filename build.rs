use std::{env, process::Command};

fn main() {
    let profile = env::var("PROFILE").unwrap_or_else(|_| "dev".into());

    let is_release = profile.contains("release");
    let node_env = if is_release {
        "production"
    } else {
        "development"
    };

    let mut tw = Command::new("npx");
    tw.args([
        "@tailwindcss/cli",
        "-i",
        "input.css",
        "-o",
        "assets/tailwind.css",
    ]);
    if is_release {
        tw.arg("--minify");
    }
    let status = tw
        .env("NODE_ENV", node_env)
        .status()
        .expect("tailwind failed");
    assert!(status.success(), "tailwind build failed");

    let mut sass = Command::new("npx");
    sass.args(["sass", "assets/scss/main.scss", "assets/main.css"]);
    if is_release {
        sass.args(["--style=compressed", "--no-source-map"]);
    } else {
        sass.args(["--style=expanded", "--embed-source-map"]);
    }
    let status = sass
        .env("NODE_ENV", node_env)
        .status()
        .expect("sass failed");
    assert!(status.success(), "sass build failed");
}
