#[derive(Clone, PartialEq, Debug)]
pub enum Language {
    Chinese,
    English,
}

// 全局语言状态
static LANGUAGE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

impl Language {
    pub fn from_accept_language(accept_language: &str) -> Self {
        if accept_language.contains("zh") {
            Language::Chinese
        } else {
            Language::English
        }
    }
    
    pub fn code(&self) -> &'static str {
        match self {
            Language::Chinese => "zh",
            Language::English => "en",
        }
    }
    
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Language::Chinese,
            1 => Language::English,
            _ => Language::Chinese,
        }
    }
    
    pub fn to_u8(&self) -> u8 {
        match self {
            Language::Chinese => 0,
            Language::English => 1,
        }
    }
    
    pub fn set_global(language: Language) {
        LANGUAGE.store(language.to_u8(), std::sync::atomic::Ordering::Relaxed);
    }
    
    pub fn get_global() -> Language {
        Language::from_u8(LANGUAGE.load(std::sync::atomic::Ordering::Relaxed))
    }
}

pub struct I18nContext {
    pub language: Language,
}

impl I18nContext {
    pub fn new(language: Language) -> Self {
        Self { language }
    }
    
    pub fn t(&self, key: &str) -> &'static str {
        match self.language {
            Language::Chinese => get_chinese_text(key),
            Language::English => get_english_text(key),
        }
    }
}

// 中文文本
fn get_chinese_text(key: &str) -> &'static str {
    match key {
        // 导航栏
        "nav.home" => "首页",
        "nav.blog" => "博客",
        
        // 首页 - Hero 部分
        "home.hero.greeting" => "👋 你好，我是 TsaoLun",
        "home.hero.intro" => "🧑‍💻 全栈开发者 • 📍 深圳",
        "home.hero.description" => "专注于现代 Web 开发技术，拥有",
        "home.hero.experience" => "开发经验",
        "home.hero.github" => "🔗 GitHub 主页",
        "home.hero.blog" => "📖 阅读博客",
        
        // 首页 - 技术栈部分
        "home.tech.title" => "🛠️ 技术栈",
        "home.tech.nodejs.description" => "专注于后端 API 开发、微服务架构和实时应用。熟练使用 Express、Koa、Fastify 等框架。",
        "home.tech.go.description" => "构建高性能的分布式系统和微服务。擅长并发编程和系统设计。",
        "home.tech.rust.description" => "探索系统编程和 WebAssembly。使用 Actix、Dioxus 等现代框架开发高性能应用。",
        
        // 首页 - 最新文章部分
        "home.posts.title" => "📝 最新文章",
        "home.posts.view_all" => "查看全部",
        "home.posts.read" => "阅读",
        
        // 页脚
        "footer.built_with" => "🚀 使用",
        "footer.and" => "和",
        "footer.built" => "构建",
        "footer.rights" => "© 2024 TsaoLun. 保留所有权利。",
        
        // 博客页面
        "blog.title" => "📖 技术博客",
        "blog.subtitle" => "分享技术经验、学习心得和项目实践",
        "blog.filter_by_tag" => "🏷️ 标签分类",
        "blog.all_posts" => "全部文章",
        "blog.read_more" => "阅读全文",
        
        // 404页面
        "404.title" => "页面未找到",
        "404.message" => "抱歉，您访问的页面不存在或已被移除。",
        "404.back_home" => "返回首页",
        
        _ => "Missing Translation", // 如果找不到翻译，返回默认文本
    }
}

// 英文文本
fn get_english_text(key: &str) -> &'static str {
    match key {
        // Navigation
        "nav.home" => "Home",
        "nav.blog" => "Blog",
        
        // Home - Hero section
        "home.hero.greeting" => "👋 Hello, I'm TsaoLun",
        "home.hero.intro" => "🧑‍💻 Full-Stack Developer • 📍 Shenzhen",
        "home.hero.description" => "Focused on modern Web development technologies with",
        "home.hero.experience" => "development experience",
        "home.hero.github" => "🔗 GitHub Profile",
        "home.hero.blog" => "📖 Read Blog",
        
        // Home - Tech stack section
        "home.tech.title" => "🛠️ Tech Stack",
        "home.tech.nodejs.description" => "Focused on backend API development, microservices architecture, and real-time applications. Proficient with Express, Koa, Fastify frameworks.",
        "home.tech.go.description" => "Building high-performance distributed systems and microservices. Expert in concurrent programming and system design.",
        "home.tech.rust.description" => "Exploring systems programming and WebAssembly. Developing high-performance applications with modern frameworks like Actix and Dioxus.",
        
        // Home - Recent posts section
        "home.posts.title" => "📝 Recent Posts",
        "home.posts.view_all" => "View All",
        "home.posts.read" => "Read",
        
        // Footer
        "footer.built_with" => "🚀 Built with",
        "footer.and" => "and",
        "footer.built" => "",
        "footer.rights" => "© 2024 TsaoLun. All rights reserved.",
        
        // Blog page
        "blog.title" => "📖 Tech Blog",
        "blog.subtitle" => "Sharing technical experiences, learning insights and project practices",
        "blog.filter_by_tag" => "🏷️ Filter by Tags",
        "blog.all_posts" => "All Posts",
        "blog.read_more" => "Read More",
        
        // 404 page
        "404.title" => "Page Not Found",
        "404.message" => "Sorry, the page you are looking for does not exist or has been removed.",
        "404.back_home" => "Back to Home",
        
        _ => "Missing Translation", // If translation not found, return default text
    }
}

// Context provider for i18n
pub fn use_i18n() -> I18nContext {
    // 首次运行时检测浏览器语言并设置全局状态
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let detected_language = detect_browser_language();
        Language::set_global(detected_language);
    });
    
    let language = Language::get_global();
    I18nContext::new(language)
}

// 切换语言的辅助函数
pub fn toggle_language() {
    let current = Language::get_global();
    let new_language = match current {
        Language::Chinese => Language::English,
        Language::English => Language::Chinese,
    };
    Language::set_global(new_language);
}

// 检测浏览器语言的辅助函数
fn detect_browser_language() -> Language {
    // 在Web环境中，我们可以通过web-sys获取语言信息
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::window;
        
        // 尝试获取浏览器语言
        if let Some(window) = window() {
            let navigator = window.navigator();
            if let Some(lang) = navigator.language() {
                if lang.starts_with("zh") {
                    return Language::Chinese;
                }
            }
        }
        
        // 默认返回英文
        Language::English
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // 在非Web环境中，默认使用中文
        Language::Chinese
    }
}
