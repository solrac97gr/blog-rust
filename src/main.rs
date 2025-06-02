use self::posts::*;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web, put, delete, Result};
use blog_rust::{*, DbPool};
use crate::models::{CreatePostRequest, UpdatePostRequest};
use serde_json::json;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// CRUD Handlers
#[get("/posts")]
async fn get_all_posts(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let post_repo = new_post_repository();
    
    let posts = run_with_connection(&pool, |conn| {
        post_repo.get_posts(conn)
    });
    
    Ok(HttpResponse::Ok().json(posts))
}

#[get("/posts/{id}")]
async fn get_post_by_id(
    path: web::Path<i32>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse> {
    let post_id = path.into_inner();
    let post_repo = new_post_repository();
    
    let post = run_with_connection(&pool, |conn| {
        post_repo.get_post_by_id(post_id, conn)
    });
    
    match post {
        Some(post) => Ok(HttpResponse::Ok().json(post)),
        None => Ok(HttpResponse::NotFound().json(json!({
            "error": "Post not found"
        })))
    }
}

#[post("/posts")]
async fn create_post(
    post_data: web::Json<CreatePostRequest>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse> {
    let post_repo = new_post_repository();
    
    let new_post = run_with_connection(&pool, |conn| {
        post_repo.create_post(
            &post_data.title,
            &post_data.body,
            &post_data.slug,
            conn
        )
    });
    
    Ok(HttpResponse::Created().json(new_post))
}

#[put("/posts/{id}")]
async fn update_post(
    path: web::Path<i32>,
    post_data: web::Json<UpdatePostRequest>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse> {
    let post_id = path.into_inner();
    let post_repo = new_post_repository();
    
    // Check if post exists first
    let existing_post = run_with_connection(&pool, |conn| {
        post_repo.get_post_by_id(post_id, conn)
    });
    
    if existing_post.is_none() {
        return Ok(HttpResponse::NotFound().json(json!({
            "error": "Post not found"
        })));
    }
    
    let updated_post = run_with_connection(&pool, |conn| {
        post_repo.update_post(
            post_id,
            &post_data.title,
            &post_data.body,
            conn
        )
    });
    
    Ok(HttpResponse::Ok().json(updated_post))
}

#[delete("/posts/{id}")]
async fn delete_post(
    path: web::Path<i32>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse> {
    let post_id = path.into_inner();
    let post_repo = new_post_repository();
    
    let deleted = run_with_connection(&pool, |conn| {
        post_repo.delete_post(post_id, conn)
    });
    
    if deleted {
        Ok(HttpResponse::Ok().json(json!({
            "message": "Post deleted successfully"
        })))
    } else {
        Ok(HttpResponse::NotFound().json(json!({
            "error": "Post not found"
        })))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get a connection pool
    let pool = establish_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Pass pool to the app state
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(get_all_posts)
            .service(get_post_by_id)
            .service(create_post)
            .service(update_post)
            .service(delete_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
