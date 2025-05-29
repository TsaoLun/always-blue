use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            class: "navbar bg-base-100 shadow-lg",
            
            div {
                class: "navbar-start",
                
                Link { 
                    to: Route::Home {},
                    class: "btn btn-ghost text-xl font-bold",
                    "🚀 TsaoLun"
                }
            }
            
            div {
                class: "navbar-end",
                
                Link { 
                    to: Route::Blog {},
                    class: "btn btn-ghost",
                    "📝 博客"
                }
                
                a {
                    href: "https://github.com/tsaoLun",
                    target: "_blank",
                    class: "btn btn-ghost",
                    "💻 GitHub"
                }
            }
        }
    }
}
