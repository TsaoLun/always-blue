use dioxus::prelude::*;

mod blog;
mod components;
mod pages;
mod i18n;

use components::*;
use pages::{Home as HomePage, Blog as BlogPage, BlogPost as BlogPostPage, BlogTag as BlogTagPage};

fn main() {
    // Initialize logger
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(log::Level::Debug)
            .expect("Failed to initialize logger");
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    }
    
    // Launch application with appropriate configuration
    #[cfg(target_arch = "wasm32")]
    {
        dioxus::launch(app);
    }
    
    #[cfg(target_os = "ios")]
    {
        dioxus::launch(app);
    }
    
    #[cfg(target_os = "android")]
    {
        dioxus::launch(app);
    }
    
    #[cfg(not(any(target_arch = "wasm32", target_os = "ios", target_os = "android")))]
    {
        dioxus::launch(app);
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog")]
    Blog {},
    #[route("/blog/post/:slug")]
    BlogPost { slug: String },
    #[route("/blog/tag/:tag")]
    BlogTag { tag: String },
}

fn app() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Layout {
            HomePage {}
        }
    }
}

#[component]
fn Blog() -> Element {
    rsx! {
        Layout {
            BlogPage {}
        }
    }
}

#[component]
fn BlogPost(slug: String) -> Element {
    rsx! {
        Layout {
            BlogPostPage { slug }
        }
    }
}

#[component] 
fn BlogTag(tag: String) -> Element {
    rsx! {
        Layout {
            BlogTagPage { tag }
        }
    }
}