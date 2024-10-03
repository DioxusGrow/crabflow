#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

pub const TAILWIND_STYLE: &str = asset!("./assets/tailwind.css");

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {

        head::Meta { name: "description", content: "&lt;model-viewer&gt; template" }
        head::Link { rel: "stylesheet", href: TAILWIND_STYLE }
        Script { src: "https://cdn.tailwindcss.com" }
        Script { type: "module", src: "https://unpkg.com/@google/model-viewer@3.5.0/dist/model-viewer.js" }
        // Script { src: "./assets/model-viewer.min.js" }
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let test_style = r#"    
        model-viewer {
            width: 400px;
            height: 600px;
            margin: 0 auto;
        }"#;

    rsx! {
        style { r#type: "text/tailwindcss", "{test_style}" }
        model-viewer {
            class: "w-screen h-screen",
            src: "./assets/img/crabflow.glb",
            ar: true,
            "ar-modes": "webxr scene-viewer quick-look",
            "camera-controls": true,
            "tone-mapping": "neutral",
            poster: "poster.webp",
            "shadow-intensity": "1"
        }

    }
}
