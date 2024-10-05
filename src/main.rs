#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

pub const TAILWIND_STYLE: &str = asset!("./assets/tailwind.css");
pub const CRAB: &str = asset!("./assets/img/crab3d.glb");

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
        Script {
            r#type: "module",
            src: "https://ajax.googleapis.com/ajax/libs/model-viewer/3.5.0/model-viewer.min.js"
        }
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
    rsx! {
        // style { r#type: "text/tailwindcss", "{test_style}" }
        div { class: "w-screen h-screen",
            model-viewer {
                class: "w-screen h-screen",
                src: "{CRAB}",
                ar: "",
                "ar-modes": "webxr scene-viewer quick-look",
                "camera-controls": "",
                "tone-mapping": "neutral",
                poster: "poster.webp",
                "shadow-intensity": "1"
            }
        }
    }
}
