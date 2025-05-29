use dioxus::prelude::*;
use crate::{Route, i18n::{use_i18n, toggle_language, Language}};

#[component]
pub fn Navbar() -> Element {
    let i18n = use_i18n();
    
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
                    "📝 ",
                    {i18n.t("nav.blog")}
                }
                
                // 语言切换按钮
                button {
                    class: "btn btn-ghost",
                    onclick: move |_| {
                        toggle_language();
                    },
                    if i18n.language == Language::Chinese { "🌐 EN" } else { "🌐 中文" }
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
