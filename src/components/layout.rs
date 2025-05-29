use dioxus::prelude::*;
use crate::{components::Navbar, i18n::use_i18n};

#[component]
pub fn Layout(children: Element) -> Element {
    let i18n = use_i18n();
    
    rsx! {
        div { 
            class: "min-h-screen bg-base-200",
            "data-theme": "cupcake",
            
            Navbar {}
            
            main {
                {children}
            }
            
            footer {
                class: "footer footer-center p-10 bg-base-200 text-base-content",
                
                div {
                    p {
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
                        class: "text-sm",
                        {i18n.t("footer.rights")}
                    }
                }
            }
        }
    }
}
