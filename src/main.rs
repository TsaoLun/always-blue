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
            class: "container mx-auto p-4 max-w-md",
            style: "font-family: system-ui, sans-serif;",
            
            h1 { 
                class: "text-3xl font-bold text-center mb-6",
                "Dioxus 0.6 Demo"
            }
            
            div { 
                class: "bg-blue-500 text-white p-6 rounded-lg shadow-lg",
                
                h2 { 
                    class: "text-xl font-semibold mb-4 text-center",
                    "Counter: {count}"
                }
                
                div {
                    class: "flex justify-center space-x-4",
                    
                    button {
                        class: "bg-white text-blue-500 px-4 py-2 rounded-md font-semibold hover:bg-blue-100 transition-colors",
                        onclick: move |_| count += 1,
                        "Increment"
                    }
                    
                    button {
                        class: "bg-white text-blue-500 px-4 py-2 rounded-md font-semibold hover:bg-blue-100 transition-colors",
                        onclick: move |_| {
                            let current = count();
                            if current > 0 {
                                count -= 1;
                            }
                        },
                        "Decrement"
                    }
                }
            }
            
            footer {
                class: "mt-8 text-center text-gray-500 text-sm",
                "Built with Dioxus 0.6 - Deployed on Deno Deploy"
            }
        }
    }
}