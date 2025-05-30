use dioxus::prelude::*;
use crate::{Route, blog::get_posts_by_tag};

#[component] 
pub fn BlogTag(tag: String) -> Element {
    let posts = get_posts_by_tag(&tag);
    
    rsx! {
        div {
            class: "container mx-auto max-w-6xl p-4 sm:p-6",
            
            // Breadcrumb
            div {
                class: "breadcrumbs text-sm mb-4 sm:mb-6 overflow-x-auto",
                
                ul {
                    li {
                        Link { 
                            to: Route::Home {},
                            class: "text-primary hover:text-primary-focus transition-colors",
                            "首页"
                        }
                    }
                    li {
                        Link { 
                            to: Route::Blog {},
                            class: "text-primary hover:text-primary-focus transition-colors",
                            "博客"
                        }
                    }
                    li { 
                        class: "text-base-content/70",
                        "标签: {tag}" 
                    }
                }
            }
            
            // Header
            div {
                class: "text-center mb-8 sm:mb-12",
                
                h1 {
                    class: "text-2xl sm:text-3xl lg:text-4xl font-bold mb-3 sm:mb-4",
                    "🏷️ {tag}"
                }
                
                p {
                    class: "text-base sm:text-lg text-base-content/70 px-2",
                    "共找到 {posts.len()} 篇相关文章"
                }
            }
            
            if posts.is_empty() {
                div {
                    class: "text-center py-8 sm:py-16",
                    
                    h2 {
                        class: "text-xl sm:text-2xl font-bold mb-4",
                        "😅 暂无相关文章"
                    }
                    
                    Link { 
                        to: Route::Blog {},
                        class: "btn btn-primary btn-sm sm:btn-md",
                        "返回博客"
                    }
                }
            } else {
                div {
                    class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6",
                    
                    {posts.into_iter().map(|post| {
                        let slug = post.slug.clone();
                        let tags_for_links = post.tags.clone();
                        rsx! {
                            div {
                                key: "{post.slug}",
                                class: "card bg-base-100 shadow-lg hover:shadow-xl transition-all duration-300 active:scale-[0.98]",
                                
                                div {
                                    class: "card-body p-4 sm:p-6",
                                    
                                    h3 {
                                        class: "card-title text-base sm:text-lg mb-2 line-clamp-2",
                                        "{post.title}"
                                    }
                                    
                                    p {
                                        class: "text-sm text-base-content/70 mb-3 sm:mb-4 line-clamp-3",
                                        "{post.summary}"
                                    }
                                    
                                    div {
                                        class: "flex flex-wrap gap-1 mb-3 sm:mb-4",
                                        {tags_for_links.into_iter().map(|post_tag| {
                                            let post_tag_clone = post_tag.clone();
                                            rsx! {
                                                Link {
                                                    key: "{post_tag}",
                                                    to: Route::BlogTag { tag: post_tag_clone },
                                                    class: if post_tag == tag { 
                                                        "badge badge-secondary text-xs" 
                                                    } else { 
                                                        "badge badge-primary badge-sm text-xs hover:badge-secondary transition-colors active:scale-95" 
                                                    },
                                                    "{post_tag}"
                                                }
                                            }
                                        })}
                                    }
                                    
                                    div {
                                        class: "card-actions justify-between items-center mt-auto",
                                        
                                        span {
                                            class: "text-xs text-base-content/50",
                                            "{post.date}"
                                        }
                                        
                                        Link { 
                                            to: Route::BlogPost { slug },
                                            class: "btn btn-sm btn-primary active:scale-95 transition-transform",
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
