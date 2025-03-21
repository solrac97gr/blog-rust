use self::models::*;
use blog_rust::*;
use diesel::prelude::*;

fn main() {
    use self::schema::posts::dsl::*;
    let connection = &mut establish_connection();

    diesel::insert_into(posts)
        .values(NewPost {
            title: "Hello, world! 2",
            body: "This is my second post!",
            slug: "hello-world-2",
        })
        .execute(connection)
        .expect("Error saving new post");

    let results = posts
        .select(Post::as_select())
        .limit(1)
        .filter(body.is_not(""))
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
