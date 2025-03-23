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

    diesel::update(posts.filter(id.eq(2)))
        .set(title.eq("Nuevo título"))
        .execute(connection)
        .expect("Error en el update");

    let updated_post = posts
        .filter(id.eq(2))
        .first::<Post>(connection)
        .expect("Error al obtener el post actualizado");

    println!("Updated post: {:?}", updated_post.title);

    let results = posts
        .select(Post::as_select()) // Get not empty body posts
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
