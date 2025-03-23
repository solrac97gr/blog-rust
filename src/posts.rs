use crate::models::{NewPost, Post};
use diesel::QueryableByName;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;
use diesel::sqlite::SqliteConnection;

// Add this struct to handle the raw SQL query result
#[derive(QueryableByName)]
struct LastId {
    #[diesel(sql_type = Integer)]
    last_insert_rowid: i32,
}

pub fn create_post(
    post_title: &str,
    post_body: &str,
    post_slug: &str,
    connection: &mut SqliteConnection,
) -> Post {
    use crate::schema::posts::dsl::{id, posts};

    let new_post = NewPost {
        title: post_title,
        body: post_body,
        slug: post_slug,
    };

    // Use a transaction to ensure atomic operation
    connection
        .transaction(|conn| {
            // Insert the post
            diesel::insert_into(posts)
                .values(&new_post)
                .execute(conn)
                .expect("Error creating the post");

            // Get the last inserted row ID from SQLite
            let result = sql_query("SELECT last_insert_rowid() as last_insert_rowid")
                .get_result::<LastId>(conn)?;

            let last_id = result.last_insert_rowid;

            // Fetch the newly created post using the exact ID
            posts
                .filter(id.eq(last_id))
                .first(conn)
                .map_err(|e| diesel::result::Error::from(e))
        })
        .expect("Error in transaction while creating post")
}

pub fn update_post(
    post_id: i32,
    post_title: &str,
    post_body: &str,
    connection: &mut SqliteConnection,
) -> Post {
    use crate::schema::posts::dsl::{body, id, posts, title};

    // First update the post
    diesel::update(posts)
        .filter(id.eq(post_id))
        .set((title.eq(post_title), body.eq(post_body)))
        .execute(connection)
        .expect("Error updating the post");

    // Then fetch the updated post
    let post = get_post_by_id(post_id, connection);
    post.unwrap()
}

pub fn get_posts(connection: &mut SqliteConnection) -> Vec<Post> {
    use crate::schema::posts::dsl::posts;

    let results = posts
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    results
}

pub fn delete_post(post_id: i32, connection: &mut SqliteConnection) -> bool {
    use crate::schema::posts::dsl::{id, posts};

    diesel::delete(posts.filter(id.eq(post_id)))
        .execute(connection)
        .expect("Error deleting the post")
        > 0
}

pub fn get_post_by_id(post_id: i32, connection: &mut SqliteConnection) -> Option<Post> {
    use crate::schema::posts::dsl::{id, posts};

    posts
        .filter(id.eq(post_id))
        .first(connection)
        .optional()
        .expect("Error fetching the post")
}
