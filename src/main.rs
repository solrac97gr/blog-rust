use self::posts::*;
use blog_rust::*;

fn main() {
    // Create a repository instance
    let post_repo = new_post_repository();

    // Get a connection
    let connection = &mut establish_connection();

    // Create a post using the repository
    post_repo.create_post(
        "Hello, world!",
        "This is my first post!",
        "hello-world", // adding the slug parameter
        connection,
    );

    // Update a post using the repository
    let updated_post =
        post_repo.update_post(2, "Hello, world! 2", "This is my second post!", connection);
    println!("Updated post: {:?}", updated_post.title);

    // Get all posts using the repository
    let results = post_repo.get_posts(connection);
    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------");
        println!("{}", post.body);
    }

    // Other operations should also use the repository pattern
    // For example:
    // let deleted = post_repo.delete_post(1, connection);
    // let post = post_repo.get_post_by_id(1, connection);
}
