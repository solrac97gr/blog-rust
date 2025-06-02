use serde::{Deserialize, Serialize};
use crate::domain::Post;

/// DTO for creating a new post via HTTP
#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub body: String,
    pub slug: String,
}

/// DTO for updating a post via HTTP
#[derive(Deserialize)]
pub struct UpdatePostRequest {
    pub title: String,
    pub body: String,
}

/// DTO for returning post data via HTTP
#[derive(Serialize)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        PostResponse {
            id: post.id.unwrap_or(0), // This should only be called for posts with IDs
            title: post.title,
            slug: post.slug,
            body: post.body,
        }
    }
}

impl From<CreatePostRequest> for Post {
    fn from(req: CreatePostRequest) -> Self {
        Post::new(req.title, req.slug, req.body)
    }
}
