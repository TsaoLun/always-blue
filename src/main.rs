use dioxus::prelude::*;

fn main() {
    // Initialize logger
    console_log::init_with_level(log::Level::Debug)
        .expect("Failed to initialize logger");
    
    // Launch web application
    launch(app);
}

fn app() -> Element {
    let mut count = use_signal(|| 0);
    
    rsx! {
        div { 
            class: "min-h-screen bg-base-200 flex items-center justify-center p-4",
            "data-theme": "cupcake",
            
            div {
                class: "card w-96 bg-base-100 shadow-xl",
                
                div {
                    class: "card-body items-center text-center",
                    
                    h1 { 
                        class: "card-title text-3xl font-bold mb-6 text-primary",
                        "🎯 Dioxus 0.6 Demo"
                    }
                    
                    div {
                        class: "stats shadow mb-6",
                        
                        div {
                            class: "stat place-items-center",
                            
                            div {
                                class: "stat-title",
                                "Counter Value"
                            }
                            
                            div {
                                class: "stat-value text-primary",
                                "{count}"
                            }
                            
                            div {
                                class: "stat-desc",
                                "Click buttons to change"
                            }
                        }
                    }
                    
                    div {
                        class: "card-actions justify-center gap-4",
                        
                        button {
                            class: "btn btn-primary btn-lg",
                            onclick: move |_| count += 1,
                            "➕ Increment"
                        }
                        
                        button {
                            class: "btn btn-secondary btn-lg",
                            onclick: move |_| {
                                let current = count();
                                if current > 0 {
                                    count -= 1;
                                }
                            },
                            "➖ Decrement"
                        }
                    }
                    
                    div {
                        class: "divider",
                        "Actions"
                    }
                    
                    button {
                        class: "btn btn-outline btn-error btn-sm",
                        onclick: move |_| count.set(0),
                        "🔄 Reset"
                    }
                }
            }
            
            div {
                class: "toast toast-bottom toast-end",
                
                div {
                    class: "alert alert-info",
                    
                    span {
                        class: "text-xs",
                        "🚀 Built with Dioxus 0.6 + DaisyUI"
                    }
                }
            }
        }
    }
}