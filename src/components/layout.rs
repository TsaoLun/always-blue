use dioxus::prelude::*;
use crate::{components::Navbar, i18n::use_i18n};

#[component]
pub fn Layout(children: Element) -> Element {
    let i18n = use_i18n();
    
    rsx! {
        div { 
            class: "min-h-screen bg-base-200 flex flex-col",
            "data-theme": "cupcake",
            // iOS safe area support
            style: "padding-top: env(safe-area-inset-top); padding-bottom: env(safe-area-inset-bottom);",
            
            Navbar {}
            
            main {
                class: "flex-1",
                {children}
            }
            
            footer {
                class: "footer footer-center p-4 sm:p-10 bg-base-200 text-base-content mt-auto",
                
                div {
                    p {
                        class: "text-sm sm:text-base",
                        {i18n.t("footer.built_with")} " "
                        a {
                            href: "https://dioxuslabs.com/",
                            target: "_blank", 
                            class: "link link-primary",
                            "Dioxus 0.6"
                        }
                        " " {i18n.t("footer.and")} " "
                        a {
                            href: "https://daisyui.com/",
                            target: "_blank", 
                            class: "link link-primary",
                            "DaisyUI"
                        }
                        " " {i18n.t("footer.built")}
                    }
                    
                    p {
                        class: "text-xs sm:text-sm opacity-70",
                        {i18n.t("footer.rights")}
                    }
                }
            }
        }
    }
}
