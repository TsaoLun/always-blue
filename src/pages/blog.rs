use dioxus::prelude::*;
use crate::{Route, blog::{get_all_posts, get_all_tags}};

#[component]
pub fn Blog() -> Element {
    let posts = get_all_posts();
    let tags = get_all_tags();
    
    rsx! {
        div {
            class: "container mx-auto max-w-6xl p-6",
            
            // Header
            div {
                class: "text-center mb-12",
                
                h1 {
                    class: "text-4xl font-bold mb-4",
                    "📝 技术博客"
                }
                
                p {
                    class: "text-lg text-base-content/70",
                    "分享技术经验、学习心得和项目实践"
                }
            }
            
            // Tags Section
            div {
                class: "mb-8",
                
                h2 {
                    class: "text-xl font-bold mb-4",
                    "🏷️ 标签分类"
                }
                
                div {
                    class: "flex flex-wrap gap-2",
                    {tags.into_iter().map(|tag| {
                        let tag_clone = tag.clone();
                        rsx! {
                            Link {
                                key: "{tag}",
                                to: Route::BlogTag { tag: tag_clone },
                                class: "badge badge-primary badge-lg hover:badge-secondary transition-colors",
                                "{tag}"
                            }
                        }
                    })}
                }
            }
            
            // Posts Grid
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
                                    {tags_for_links.into_iter().map(|tag| {
                                        let tag_clone = tag.clone();
                                        rsx! {
                                            Link {
                                                key: "{tag}",
                                                to: Route::BlogTag { tag: tag_clone },
                                                class: "badge badge-primary badge-sm hover:badge-secondary transition-colors",
                                                "{tag}"
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
