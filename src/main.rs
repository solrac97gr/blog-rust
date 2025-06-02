use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, Result};
use blog_rust::application::PostService;
use blog_rust::infrastructure::{establish_connection_pool, SqlitePostRepository, PostHandler};
use serde_json::json;
use std::sync::Arc;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "blog-rust-hexagonal"
    }))
}

// Wrapper functions to handle the handler method calls
async fn get_all_posts_handler(
    handler: web::Data<PostHandler>
) -> Result<HttpResponse> {
    handler.get_all_posts().await
}

async fn get_post_by_id_handler(
    path: web::Path<i32>,
    handler: web::Data<PostHandler>
) -> Result<HttpResponse> {
    handler.get_post_by_id(path).await
}

async fn create_post_handler(
    post_data: web::Json<blog_rust::infrastructure::CreatePostRequest>,
    handler: web::Data<PostHandler>
) -> Result<HttpResponse> {
    handler.create_post(post_data).await
}

async fn update_post_handler(
    path: web::Path<i32>,
    post_data: web::Json<blog_rust::infrastructure::UpdatePostRequest>,
    handler: web::Data<PostHandler>
) -> Result<HttpResponse> {
    handler.update_post(path, post_data).await
}

async fn delete_post_handler(
    path: web::Path<i32>,
    handler: web::Data<PostHandler>
) -> Result<HttpResponse> {
    handler.delete_post(path).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting Blog Rust Server with Hexagonal Architecture...");

    // Infrastructure Layer: Database connection
    let pool = establish_connection_pool();
    let pool_arc = Arc::new(pool);

    // Infrastructure Layer: Repository implementation
    let post_repository = SqlitePostRepository::new(Arc::clone(&pool_arc));
    let post_repository_arc: Arc<dyn blog_rust::domain::PostRepository> = Arc::new(post_repository);

    // Application Layer: Service/Use Cases
    let post_service = Arc::new(PostService::new(post_repository_arc));

    // Infrastructure Layer: Web handlers
    let post_handler = PostHandler::new(post_service);

    println!("✅ Dependencies injected successfully");
    println!("🌐 Server starting on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(post_handler.clone()))
            .service(hello)
            .service(health_check)
            .route("/posts", web::get().to(get_all_posts_handler))
            .route("/posts/{id}", web::get().to(get_post_by_id_handler))
            .route("/posts", web::post().to(create_post_handler))
            .route("/posts/{id}", web::put().to(update_post_handler))
            .route("/posts/{id}", web::delete().to(delete_post_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
