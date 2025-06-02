use crate::domain::{Post, PostRepository};
use std::sync::Arc;

/// Application service that orchestrates business operations
/// This layer contains the use cases and application-specific business rules
pub struct PostService {
    repository: Arc<dyn PostRepository>,
}

impl PostService {
    pub fn new(repository: Arc<dyn PostRepository>) -> Self {
        Self { repository }
    }

    /// Get all posts use case
    pub async fn get_all_posts(&self) -> Result<Vec<Post>, String> {
        self.repository.find_all().await
    }

    /// Get post by ID use case
    pub async fn get_post_by_id(&self, id: i32) -> Result<Option<Post>, String> {
        if id <= 0 {
            return Err("Invalid post ID".to_string());
        }
        self.repository.find_by_id(id).await
    }

    /// Create new post use case
    pub async fn create_post(&self, title: String, slug: String, body: String) -> Result<Post, String> {
        let post = Post::new(title, slug, body);
        
        // Validate the post
        post.validate()?;
        
        // Check if slug already exists (business rule)
        // Note: In a real application, you might want to add a find_by_slug method
        
        self.repository.save(post).await
    }

    /// Update post use case
    pub async fn update_post(&self, id: i32, title: String, body: String) -> Result<Option<Post>, String> {
        if id <= 0 {
            return Err("Invalid post ID".to_string());
        }

        // First check if post exists
        let existing_post = self.repository.find_by_id(id).await?;
        
        match existing_post {
            Some(mut post) => {
                post.update(title, body);
                post.validate()?;
                self.repository.update(id, post).await
            }
            None => Ok(None)
        }
    }

    /// Delete post use case
    pub async fn delete_post(&self, id: i32) -> Result<bool, String> {
        if id <= 0 {
            return Err("Invalid post ID".to_string());
        }
        self.repository.delete(id).await
    }
}
