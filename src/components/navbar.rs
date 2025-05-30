use dioxus::prelude::*;
use crate::{Route, i18n::{use_i18n, toggle_language, Language}};

#[component]
pub fn Navbar() -> Element {
    let i18n = use_i18n();
    let mut is_menu_open = use_signal(|| false);
    
    rsx! {
        nav {
            class: "navbar bg-base-100 shadow-lg px-2 sm:px-4",
            
            div {
                class: "navbar-start",
                
                // Mobile menu button
                div {
                    class: "dropdown lg:hidden",
                    
                    button {
                        class: "btn btn-square btn-ghost",
                        onclick: move |_| {
                            is_menu_open.set(!is_menu_open());
                        },
                        svg {
                            class: "w-6 h-6",
                            fill: "none",
                            stroke: "currentColor",
                            "viewBox": "0 0 24 24",
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                "stroke-width": "2",
                                d: "M4 6h16M4 12h16M4 18h16"
                            }
                        }
                    }
                    
                    if is_menu_open() {
                        ul {
                            class: "menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52",
                            
                            li {
                                Link { 
                                    to: Route::Blog {},
                                    class: "text-base font-medium",
                                    onclick: move |_| is_menu_open.set(false),
                                    "📝 {i18n.t(\"nav.blog\")}"
                                }
                            }
                            
                            li {
                                button {
                                    class: "text-base font-medium",
                                    onclick: move |_| {
                                        toggle_language();
                                        is_menu_open.set(false);
                                    },
                                    if i18n.language == Language::Chinese { "🌐 EN" } else { "🌐 中文" }
                                }
                            }
                            
                            li {
                                a {
                                    href: "https://github.com/tsaoLun",
                                    target: "_blank",
                                    class: "text-base font-medium",
                                    onclick: move |_| is_menu_open.set(false),
                                    "💻 GitHub"
                                }
                            }
                        }
                    }
                }
                
                Link { 
                    to: Route::Home {},
                    class: "btn btn-ghost text-lg sm:text-xl font-bold",
                    onclick: move |_| is_menu_open.set(false),
                    "🚀 TsaoLun"
                }
            }
            
            div {
                class: "navbar-end hidden lg:flex",
                
                ul {
                    class: "menu menu-horizontal px-1",
                    
                    li {
                        Link { 
                            to: Route::Blog {},
                            class: "btn btn-ghost",
                            "📝 {i18n.t(\"nav.blog\")}"
                        }
                    }
                    
                    li {
                        button {
                            class: "btn btn-ghost",
                            onclick: move |_| {
                                toggle_language();
                            },
                            if i18n.language == Language::Chinese { "🌐 EN" } else { "🌐 中文" }
                        }
                    }
                    
                    li {
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
    }
}
