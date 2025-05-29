use dioxus::prelude::*;
use crate::{Route, blog::get_all_posts};

#[component]
pub fn Home() -> Element {
    rsx! {
        // Hero Section
        section {
            class: "hero min-h-[80vh] bg-gradient-to-br from-primary/20 to-secondary/20",
            
            div {
                class: "hero-content text-center",
                
                div {
                    class: "max-w-4xl",
                    
                    h1 { 
                        class: "text-5xl font-bold mb-6 bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent",
                        "👋 你好，我是 TsaoLun"
                    }
                    
                    div {
                        class: "text-xl mb-8 leading-relaxed",
                        
                        p {
                            class: "mb-4",
                            "🧑‍💻 全栈开发者 • 📍 深圳"
                        }
                        
                        p {
                            class: "mb-6",
                            "专注于现代 Web 开发技术，拥有 "
                            span { class: "badge badge-primary", "Node.js" }
                            "、"
                            span { class: "badge badge-secondary", "Go" }
                            " 和 "
                            span { class: "badge badge-accent", "Rust" }
                            " 开发经验"
                        }
                    }
                    
                    div {
                        class: "flex flex-wrap justify-center gap-4 mb-8",
                        
                        a {
                            href: "https://github.com/tsaoLun",
                            target: "_blank",
                            class: "btn btn-primary btn-lg",
                            "🔗 GitHub 主页"
                        }
                        
                        Link { 
                            to: Route::Blog {},
                            class: "btn btn-outline btn-lg",
                            "📖 阅读博客"
                        }
                    }
                }
            }
        }
        
        // Tech Stack Section
        section {
            class: "py-20 px-4",
            
            div {
                class: "container mx-auto max-w-6xl",
                
                h2 {
                    class: "text-3xl font-bold text-center mb-12",
                    "🛠️ 技术栈"
                }
                
                div {
                    class: "grid md:grid-cols-3 gap-8",
                    
                    // Node.js Card
                    div {
                        class: "card bg-base-100 shadow-xl",
                        
                        div {
                            class: "card-body items-center text-center",
                            
                            div {
                                class: "text-6xl mb-4",
                                "🟢"
                            }
                            
                            h3 {
                                class: "card-title text-2xl mb-4",
                                "Node.js"
                            }
                            
                            p {
                                class: "text-base-content/70",
                                "专注于后端 API 开发、微服务架构和实时应用。熟练使用 Express、Koa、Fastify 等框架。"
                            }
                            
                            div {
                                class: "flex flex-wrap gap-2 mt-4",
                                span { class: "badge badge-outline", "Express" }
                                span { class: "badge badge-outline", "Fastify" }
                                span { class: "badge badge-outline", "TypeScript" }
                            }
                        }
                    }
                    
                    // Go Card
                    div {
                        class: "card bg-base-100 shadow-xl",
                        
                        div {
                            class: "card-body items-center text-center",
                            
                            div {
                                class: "text-6xl mb-4",
                                "🔷"
                            }
                            
                            h3 {
                                class: "card-title text-2xl mb-4",
                                "Go"
                            }
                            
                            p {
                                class: "text-base-content/70",
                                "构建高性能的分布式系统和微服务。擅长并发编程和系统设计。"
                            }
                            
                            div {
                                class: "flex flex-wrap gap-2 mt-4",
                                span { class: "badge badge-outline", "Gin" }
                                span { class: "badge badge-outline", "gRPC" }
                                span { class: "badge badge-outline", "Docker" }
                            }
                        }
                    }
                    
                    // Rust Card
                    div {
                        class: "card bg-base-100 shadow-xl",
                        
                        div {
                            class: "card-body items-center text-center",
                            
                            div {
                                class: "text-6xl mb-4",
                                "🦀"
                            }
                            
                            h3 {
                                class: "card-title text-2xl mb-4",
                                "Rust"
                            }
                            
                            p {
                                class: "text-base-content/70",
                                "探索系统编程和 WebAssembly。使用 Actix、Dioxus 等现代框架开发高性能应用。"
                            }
                            
                            div {
                                class: "flex flex-wrap gap-2 mt-4",
                                span { class: "badge badge-outline", "Actix-web" }
                                span { class: "badge badge-outline", "Dioxus" }
                                span { class: "badge badge-outline", "WebAssembly" }
                            }
                        }
                    }
                }
            }
        }
        
        // Recent Posts Section
        section {
            class: "py-20 px-4 bg-base-100",
            
            div {
                class: "container mx-auto max-w-6xl",
                
                div {
                    class: "flex justify-between items-center mb-12",
                    
                    h2 {
                        class: "text-3xl font-bold",
                        "📝 最新文章"
                    }
                    
                    Link { 
                        to: Route::Blog {},
                        class: "btn btn-primary",
                        "查看全部"
                    }
                }
                
                div {
                    class: "grid md:grid-cols-2 lg:grid-cols-3 gap-6",
                    
                    {get_all_posts().into_iter().take(3).map(|post| {
                        let slug = post.slug.clone();
                        rsx! {
                            div {
                                key: "{post.slug}",
                                class: "card bg-base-200 shadow-lg hover:shadow-xl transition-shadow",
                                
                                div {
                                    class: "card-body",
                                    
                                    h3 {
                                        class: "card-title text-lg mb-2",
                                        "{post.title}"
                                    }
                                    
                                    p {
                                        class: "text-sm text-base-content/70 mb-4",
                                        "{post.summary}"
                                    }
                                    
                                    div {
                                        class: "flex flex-wrap gap-1 mb-4",
                                        {post.tags.into_iter().map(|tag| rsx! {
                                            span {
                                                key: "{tag}",
                                                class: "badge badge-primary badge-sm",
                                                "{tag}"
                                            }
                                        })}
                                    }
                                    
                                    div {
                                        class: "card-actions justify-between items-center",
                                        
                                        span {
                                            class: "text-xs text-base-content/50",
                                            "{post.date}"
                                        }
                                        
                                        Link { 
                                            to: Route::BlogPost { slug },
                                            class: "btn btn-sm btn-primary",
                                            "阅读"
                                        }
                                    }
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}
