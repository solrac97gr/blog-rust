use self::posts::*;
use blog_rust::*;

fn main() {
    let connection = &mut establish_connection();

    create_post(
        "Hello, world!",
        "This is my first post!",
        "hello-world",
        connection,
    );

    let updated_post = update_post(2, "Hello, world! 2", "This is my second post!", connection);

    println!("Updated post: {:?}", updated_post.title);

    let results = get_posts(connection);
    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
