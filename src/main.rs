use dioxus::prelude::*;

mod blog;
mod components;
mod pages;

use components::*;
use pages::{Home as HomePage, Blog as BlogPage, BlogPost as BlogPostPage, BlogTag as BlogTagPage};

fn main() {
    // Initialize logger
    console_log::init_with_level(log::Level::Debug)
        .expect("Failed to initialize logger");
    
    // Launch web application
    launch(app);
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