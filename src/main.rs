use self::posts::*;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use blog_rust::*;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create a repository instance
    let post_repo = new_post_repository();

    // Get a connection pool
    let pool = establish_connection_pool();

    // Create a post using the repository
    let new_post = run_with_connection(&pool, |conn| {
        post_repo.create_post(
            "Hello, world!",
            "This is my first post!",
            "hello-world",
            conn,
        )
    });
    println!("Created post: {}", new_post.title);

    // Update a post using the repository
    let updated_post = run_with_connection(&pool, |conn| {
        post_repo.update_post(2, "Hello, world! 2", "This is my second post!", conn)
    });
    println!("Updated post: {}", updated_post.title);

    // Get all posts using the repository
    let results = run_with_connection(&pool, |conn| post_repo.get_posts(conn));
    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------");
        println!("{}", post.body);
    }

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone()) // Pass pool to the app state
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
