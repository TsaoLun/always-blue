use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPost {
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub summary: String,
    pub content: String,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostMetadata {
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub summary: String,
}

impl BlogPost {
    pub fn parse_from_markdown(content: &str, slug: String) -> Result<Self, String> {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        
        if parts.len() < 3 {
            return Err("Invalid markdown format: missing frontmatter".to_string());
        }

        // Parse frontmatter (YAML-like format)
        let frontmatter = parts[1].trim();
        let content = parts[2].trim();
        
        let metadata = Self::parse_frontmatter(frontmatter)?;
        
        // Convert markdown to HTML
        let parser = pulldown_cmark::Parser::new(content);
        let mut html_content = String::new();
        pulldown_cmark::html::push_html(&mut html_content, parser);
        
        Ok(BlogPost {
            title: metadata.title,
            date: metadata.date,
            tags: metadata.tags,
            summary: metadata.summary,
            content: html_content,
            slug,
        })
    }
    
    fn parse_frontmatter(frontmatter: &str) -> Result<PostMetadata, String> {
        let mut title = String::new();
        let mut date = String::new();
        let mut tags = Vec::new();
        let mut summary = String::new();
        
        for line in frontmatter.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            if let Some(colon_pos) = line.find(':') {
                let key = line[..colon_pos].trim();
                let value = line[colon_pos + 1..].trim();
                
                match key {
                    "title" => {
                        title = value.trim_matches('"').to_string();
                    }
                    "date" => {
                        date = value.trim_matches('"').to_string();
                    }
                    "summary" => {
                        summary = value.trim_matches('"').to_string();
                    }
                    "tags" => {
                        // Parse array format: ["tag1", "tag2"]
                        let value = value.trim_matches(['[', ']']);
                        tags = value
                            .split(',')
                            .map(|tag| tag.trim().trim_matches('"').to_string())
                            .filter(|tag| !tag.is_empty())
                            .collect();
                    }
                    _ => {}
                }
            }
        }
        
        Ok(PostMetadata {
            title,
            date,
            tags,
            summary,
        })
    }
}

// Mock data for now (in a real app, you'd read from files)
pub fn get_all_posts() -> Vec<BlogPost> {
    vec![
        BlogPost {
            slug: "welcome".to_string(),
            title: "欢迎来到我的博客".to_string(),
            date: "2024-01-30".to_string(),
            tags: vec!["介绍".to_string(), "博客".to_string()],
            summary: "博客的第一篇文章，介绍博客的目标和内容".to_string(),
            content: include_str!("posts/01-welcome.md").to_string(),
        },
        BlogPost {
            slug: "nodejs-optimization".to_string(),
            title: "Node.js 性能优化实战".to_string(),
            date: "2024-01-30".to_string(),
            tags: vec!["Node.js".to_string(), "性能优化".to_string(), "实战".to_string()],
            summary: "Node.js 应用性能优化的实用技巧和工具".to_string(),
            content: include_str!("posts/04-nodejs-optimization.md").to_string(),
        },
        BlogPost {
            slug: "go-concurrency".to_string(),
            title: "Go 并发编程最佳实践".to_string(),
            date: "2024-01-25".to_string(),
            tags: vec!["Go".to_string(), "并发编程".to_string(), "最佳实践".to_string()],
            summary: "分享 Go 语言并发编程的最佳实践和常见陷阱".to_string(),
            content: include_str!("posts/03-go-concurrency.md").to_string(),
        },
        BlogPost {
            slug: "rust-web".to_string(),
            title: "Rust 在 Web 开发中的应用".to_string(),
            date: "2024-01-20".to_string(),
            tags: vec!["Rust".to_string(), "Web开发".to_string(), "技术分享".to_string()],
            summary: "探讨 Rust 在现代 Web 开发中的优势和应用场景".to_string(),
            content: include_str!("posts/02-rust-web.md").to_string(),
        },
    ]
}

pub fn get_post_by_slug(slug: &str) -> Option<BlogPost> {
    get_all_posts().into_iter().find(|post| post.slug == slug)
}

pub fn get_all_tags() -> Vec<String> {
    let mut tags = std::collections::HashSet::new();
    for post in get_all_posts() {
        for tag in post.tags {
            tags.insert(tag);
        }
    }
    let mut tags: Vec<String> = tags.into_iter().collect();
    tags.sort();
    tags
}

pub fn get_posts_by_tag(tag: &str) -> Vec<BlogPost> {
    get_all_posts()
        .into_iter()
        .filter(|post| post.tags.contains(&tag.to_string()))
        .collect()
}
