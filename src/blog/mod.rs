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

// Load and parse blog posts from markdown files
pub fn get_all_posts() -> Vec<BlogPost> {
    let mut posts = Vec::new();
    
    // Parse each markdown file
    if let Ok(post) = BlogPost::parse_from_markdown(
        include_str!("posts/01-welcome.md"), 
        "welcome".to_string()
    ) {
        posts.push(post);
    }
    
    if let Ok(post) = BlogPost::parse_from_markdown(
        include_str!("posts/02-rust-web.md"), 
        "rust-web".to_string()
    ) {
        posts.push(post);
    }
    
    if let Ok(post) = BlogPost::parse_from_markdown(
        include_str!("posts/03-go-concurrency.md"), 
        "go-concurrency".to_string()
    ) {
        posts.push(post);
    }
    
    if let Ok(post) = BlogPost::parse_from_markdown(
        include_str!("posts/04-nodejs-optimization.md"), 
        "nodejs-optimization".to_string()
    ) {
        posts.push(post);
    }
    
    // Sort by date (newest first)
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    
    posts
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
