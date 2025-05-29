use dioxus::prelude::*;
use crate::{Route, blog::get_post_by_slug};

#[component]
pub fn BlogPost(slug: String) -> Element {
    let post = get_post_by_slug(&slug);
    
    match post {
        Some(post) => rsx! {
            article {
                class: "container mx-auto max-w-4xl p-6",
                
                // Breadcrumb
                div {
                    class: "breadcrumbs text-sm mb-6",
                    
                    ul {
                        li {
                            Link { 
                                to: Route::Home {},
                                "首页"
                            }
                        }
                        li {
                            Link { 
                                to: Route::Blog {},
                                "博客"
                            }
                        }
                        li { "{post.title}" }
                    }
                }
                
                // Article Header
                header {
                    class: "mb-8",
                    
                    h1 {
                        class: "text-4xl font-bold mb-4",
                        "{post.title}"
                    }
                    
                    div {
                        class: "flex flex-wrap items-center gap-4 mb-4",
                        
                        span {
                            class: "text-base-content/70",
                            "📅 {post.date}"
                        }
                        
                        div {
                            class: "flex flex-wrap gap-1",
                            {post.tags.into_iter().map(|tag| {
                                let tag_clone = tag.clone();
                                rsx! {
                                    Link {
                                        key: "{tag}",
                                        to: Route::BlogTag { tag: tag_clone },
                                        class: "badge badge-primary hover:badge-secondary transition-colors",
                                        "{tag}"
                                    }
                                }
                            })}
                        }
                    }
                    
                    div {
                        class: "divider"
                    }
                }
                
                // Article Content
                div {
                    class: "prose prose-lg max-w-none prose-headings:text-base-content prose-p:text-base-content prose-a:text-primary prose-strong:text-base-content prose-code:text-accent prose-pre:bg-base-300 prose-blockquote:text-base-content/80",
                    dangerous_inner_html: "{post.content}"
                }
                
                // Back to Blog
                div {
                    class: "mt-12 text-center",
                    
                    Link { 
                        to: Route::Blog {},
                        class: "btn btn-primary",
                        "← 返回博客"
                    }
                }
            }
        },
        None => rsx! {
            div {
                class: "min-h-screen flex items-center justify-center",
                
                div {
                    class: "text-center",
                    
                    h1 {
                        class: "text-4xl font-bold mb-4",
                        "😅 文章未找到"
                    }
                    
                    p {
                        class: "text-lg mb-6",
                        "抱歉，您查找的文章不存在。"
                    }
                    
                    Link { 
                        to: Route::Blog {},
                        class: "btn btn-primary",
                        "返回博客"
                    }
                }
            }
        }
    }
}
