use self::models::*;
use blog_rust::*;
use diesel::prelude::*;

fn main() {
    use self::schema::posts::dsl::*;
    let connection = &mut establish_connection();

    let results = posts
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
