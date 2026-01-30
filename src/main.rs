#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus::desktop::{Config, WindowBuilder, LogicalSize};

// Modules
pub mod components;
pub mod models;
pub mod hooks;

use crate::components::dashboard::*;
use crate::models::bleuio::find_bleuio;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
static CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(target_os = "macos")]
    macos_app_nap::prevent(); 

    // 1. Define your window configuration
    let window = WindowBuilder::new()
        .with_title("Sensor Dashboard")
        .with_inner_size(LogicalSize::new(1100.0, 600.0)); // Width, Height

    // 2. Launch with the custom config
    LaunchBuilder::new()
        .with_cfg(Config::new().with_window(window))
        .launch(App);
}

#[component]
fn App() -> Element {
    let port_name = find_bleuio();
    rsx! {
        // Länka till den kompilerade filen (dx lägger den i assets/main.css som standard)
        document::Stylesheet { href: CSS }

        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Dashboard { port_name }
    }
}
