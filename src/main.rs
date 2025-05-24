use dioxus::prelude::*;

mod blog;
use blog::*;

fn main() {
    // Initialize logger
    console_log::init_with_level(log::Level::Debug)
        .expect("Failed to initialize logger");
    
    // Launch web application
    launch(app);
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog")]
    Blog {},
    #[route("/blog/post/:slug")]
    BlogPost { slug: String },
    #[route("/blog/tag/:tag")]
    BlogTag { tag: String },
}

fn app() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { 
            class: "min-h-screen bg-base-200",
            "data-theme": "cupcake",
            
            // Header/Navigation
            nav {
                class: "navbar bg-base-100 shadow-lg",
                
                div {
                    class: "navbar-start",
                    
                    Link { 
                        to: Route::Home {},
                        class: "btn btn-ghost text-xl font-bold",
                        "🚀 TsaoLun"
                    }
                }
                
                div {
                    class: "navbar-end",
                    
                    Link { 
                        to: Route::Blog {},
                        class: "btn btn-ghost",
                        "📝 博客"
                    }
                    
                    a {
                        href: "https://github.com/tsaoLun",
                        target: "_blank",
                        class: "btn btn-ghost",
                        "💻 GitHub"
                    }
                }
            }
            
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
            
            // Footer
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

#[component]
fn Blog() -> Element {
    let posts = get_all_posts();
    let tags = get_all_tags();
    
    rsx! {
        div { 
            class: "min-h-screen bg-base-200",
            "data-theme": "cupcake",
            
            // Header/Navigation
            nav {
                class: "navbar bg-base-100 shadow-lg",
                
                div {
                    class: "navbar-start",
                    
                    Link { 
                        to: Route::Home {},
                        class: "btn btn-ghost text-xl font-bold",
                        "🚀 TsaoLun"
                    }
                }
                
                div {
                    class: "navbar-end",
                    
                    Link { 
                        to: Route::Blog {},
                        class: "btn btn-ghost",
                        "📝 博客"
                    }
                    
                    a {
                        href: "https://github.com/tsaoLun",
                        target: "_blank",
                        class: "btn btn-ghost",
                        "💻 GitHub"
                    }
                }
            }
            
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
}

#[component]
fn BlogPost(slug: String) -> Element {
    let post = get_post_by_slug(&slug);
    
    match post {
        Some(post) => rsx! {
            div { 
                class: "min-h-screen bg-base-200",
                "data-theme": "cupcake",
                
                // Header/Navigation
                nav {
                    class: "navbar bg-base-100 shadow-lg",
                    
                    div {
                        class: "navbar-start",
                        
                        Link { 
                            to: Route::Home {},
                            class: "btn btn-ghost text-xl font-bold",
                            "🚀 TsaoLun"
                        }
                    }
                    
                    div {
                        class: "navbar-end",
                        
                        Link { 
                            to: Route::Blog {},
                            class: "btn btn-ghost",
                            "📝 博客"
                        }
                        
                        a {
                            href: "https://github.com/tsaoLun",
                            target: "_blank",
                            class: "btn btn-ghost",
                            "💻 GitHub"
                        }
                    }
                }
                
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
                        class: "prose prose-lg max-w-none",
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
            }
        },
        None => rsx! {
            div { 
                class: "min-h-screen bg-base-200 flex items-center justify-center",
                "data-theme": "cupcake",
                
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

#[component] 
fn BlogTag(tag: String) -> Element {
    let posts = get_posts_by_tag(&tag);
    
    rsx! {
        div { 
            class: "min-h-screen bg-base-200",
            "data-theme": "cupcake",
            
            // Header/Navigation
            nav {
                class: "navbar bg-base-100 shadow-lg",
                
                div {
                    class: "navbar-start",
                    
                    Link { 
                        to: Route::Home {},
                        class: "btn btn-ghost text-xl font-bold",
                        "🚀 TsaoLun"
                    }
                }
                
                div {
                    class: "navbar-end",
                    
                    Link { 
                        to: Route::Blog {},
                        class: "btn btn-ghost",
                        "📝 博客"
                    }
                    
                    a {
                        href: "https://github.com/tsaoLun",
                        target: "_blank",
                        class: "btn btn-ghost",
                        "💻 GitHub"
                    }
                }
            }
            
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
}