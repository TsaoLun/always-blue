use dioxus::prelude::*;
use crate::{Route, blog::get_posts_by_tag};

#[component] 
pub fn BlogTag(tag: String) -> Element {
    let posts = get_posts_by_tag(&tag);
    
    rsx! {
        div {
            class: "container mx-auto max-w-6xl p-6",
            
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
                    li { "标签: {tag}" }
                }
            }
            
            // Header
            div {
                class: "text-center mb-12",
                
                h1 {
                    class: "text-4xl font-bold mb-4",
                    "🏷️ {tag}"
                }
                
                p {
                    class: "text-lg text-base-content/70",
                    "共找到 {posts.len()} 篇相关文章"
                }
            }
            
            if posts.is_empty() {
                div {
                    class: "text-center",
                    
                    h2 {
                        class: "text-2xl font-bold mb-4",
                        "😅 暂无相关文章"
                    }
                    
                    Link { 
                        to: Route::Blog {},
                        class: "btn btn-primary",
                        "返回博客"
                    }
                }
            } else {
                div {
                    class: "grid md:grid-cols-2 lg:grid-cols-3 gap-6",
                    
                    {posts.into_iter().map(|post| {
                        let slug = post.slug.clone();
                        let tags_for_links = post.tags.clone();
                        rsx! {
                            div {
                                key: "{post.slug}",
                                class: "card bg-base-100 shadow-lg hover:shadow-xl transition-shadow",
                                
                                div {
                                    class: "card-body",
                                    
                                    h3 {
                                        class: "card-title mb-2",
                                        "{post.title}"
                                    }
                                    
                                    p {
                                        class: "text-sm text-base-content/70 mb-4",
                                        "{post.summary}"
                                    }
                                    
                                    div {
                                        class: "flex flex-wrap gap-1 mb-4",
                                        {tags_for_links.into_iter().map(|post_tag| {
                                            let post_tag_clone = post_tag.clone();
                                            rsx! {
                                                Link {
                                                    key: "{post_tag}",
                                                    to: Route::BlogTag { tag: post_tag_clone },
                                                    class: if post_tag == tag { "badge badge-secondary" } else { "badge badge-primary badge-sm hover:badge-secondary transition-colors" },
                                                    "{post_tag}"
                                                }
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
                                            "阅读全文"
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
