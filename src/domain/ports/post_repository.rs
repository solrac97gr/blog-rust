use crate::domain::entities::Post;
use async_trait::async_trait;

/// Port (interface) for post repository
/// This defines what operations the domain needs from the persistence layer
#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Post>, String>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Post>, String>;
    async fn save(&self, post: Post) -> Result<Post, String>;
    async fn update(&self, id: i32, post: Post) -> Result<Option<Post>, String>;
    async fn delete(&self, id: i32) -> Result<bool, String>;
}
