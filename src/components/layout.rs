use dioxus::prelude::*;
use crate::components::Navbar;

#[component]
pub fn Layout(children: Element) -> Element {
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
                        "🚀 Built with "
                        a {
                            href: "https://dioxuslabs.com/",
                            target: "_blank", 
                            class: "link link-primary",
                            "Dioxus 0.6"
                        }
                        " + "
                        a {
                            href: "https://daisyui.com/",
                            target: "_blank", 
                            class: "link link-primary",
                            "DaisyUI"
                        }
                    }
                    
                    p {
                        class: "text-sm",
                        "© 2024 TsaoLun. All rights reserved."
                    }
                }
            }
        }
    }
}
