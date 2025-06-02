use crate::application::PostService;
use crate::infrastructure::web::models::{CreatePostRequest, UpdatePostRequest, PostResponse};
use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use std::sync::Arc;

/// HTTP handlers for post endpoints
/// This is the adapter that translates HTTP requests to use case calls
#[derive(Clone)]
pub struct PostHandler {
    post_service: Arc<PostService>,
}

impl PostHandler {
    pub fn new(post_service: Arc<PostService>) -> Self {
        Self { post_service }
    }

    /// GET /posts - Get all posts
    pub async fn get_all_posts(&self) -> Result<HttpResponse> {
        match self.post_service.get_all_posts().await {
            Ok(posts) => {
                let responses: Vec<PostResponse> = posts.into_iter().map(PostResponse::from).collect();
                Ok(HttpResponse::Ok().json(responses))
            }
            Err(error) => Ok(HttpResponse::InternalServerError().json(json!({
                "error": error
            })))
        }
    }

    /// GET /posts/{id} - Get post by ID
    pub async fn get_post_by_id(&self, path: web::Path<i32>) -> Result<HttpResponse> {
        let post_id = path.into_inner();
        
        match self.post_service.get_post_by_id(post_id).await {
            Ok(Some(post)) => Ok(HttpResponse::Ok().json(PostResponse::from(post))),
            Ok(None) => Ok(HttpResponse::NotFound().json(json!({
                "error": "Post not found"
            }))),
            Err(error) => Ok(HttpResponse::InternalServerError().json(json!({
                "error": error
            })))
        }
    }

    /// POST /posts - Create new post
    pub async fn create_post(&self, post_data: web::Json<CreatePostRequest>) -> Result<HttpResponse> {
        let request = post_data.into_inner();
        
        match self.post_service.create_post(request.title, request.slug, request.body).await {
            Ok(post) => Ok(HttpResponse::Created().json(PostResponse::from(post))),
            Err(error) => Ok(HttpResponse::BadRequest().json(json!({
                "error": error
            })))
        }
    }

    /// PUT /posts/{id} - Update post
    pub async fn update_post(
        &self,
        path: web::Path<i32>,
        post_data: web::Json<UpdatePostRequest>
    ) -> Result<HttpResponse> {
        let post_id = path.into_inner();
        let request = post_data.into_inner();
        
        match self.post_service.update_post(post_id, request.title, request.body).await {
            Ok(Some(post)) => Ok(HttpResponse::Ok().json(PostResponse::from(post))),
            Ok(None) => Ok(HttpResponse::NotFound().json(json!({
                "error": "Post not found"
            }))),
            Err(error) => Ok(HttpResponse::BadRequest().json(json!({
                "error": error
            })))
        }
    }

    /// DELETE /posts/{id} - Delete post
    pub async fn delete_post(&self, path: web::Path<i32>) -> Result<HttpResponse> {
        let post_id = path.into_inner();
        
        match self.post_service.delete_post(post_id).await {
            Ok(true) => Ok(HttpResponse::Ok().json(json!({
                "message": "Post deleted successfully"
            }))),
            Ok(false) => Ok(HttpResponse::NotFound().json(json!({
                "error": "Post not found"
            }))),
            Err(error) => Ok(HttpResponse::InternalServerError().json(json!({
                "error": error
            })))
        }
    }
}
