use serde::{Deserialize, Serialize};

/// Domain entity representing a blog post
/// This is the core business object, independent of any external concerns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub slug: String,
    pub body: String,
}

impl Post {
    /// Create a new post without an ID (for creation)
    pub fn new(title: String, slug: String, body: String) -> Self {
        Self {
            id: None,
            title,
            slug,
            body,
        }
    }

    /// Create a post with an ID (for existing posts)
    pub fn with_id(id: i32, title: String, slug: String, body: String) -> Self {
        Self {
            id: Some(id),
            title,
            slug,
            body,
        }
    }

    /// Validate the post data
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        
        if self.slug.trim().is_empty() {
            return Err("Slug cannot be empty".to_string());
        }
        
        if self.body.trim().is_empty() {
            return Err("Body cannot be empty".to_string());
        }
        
        Ok(())
    }

    /// Update the post with new data
    pub fn update(&mut self, title: String, body: String) {
        self.title = title;
        self.body = body;
    }
}
