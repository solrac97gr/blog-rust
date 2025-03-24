use crate::models::{NewPost, Post};
use diesel::QueryableByName;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;
use diesel::sqlite::SqliteConnection;

// Struct to handle the raw SQL query result
#[derive(QueryableByName)]
struct LastId {
    #[diesel(sql_type = Integer)]
    last_insert_rowid: i32,
}

// The trait (interface) definition
pub trait PostRepository {
    fn create_post(
        &self,
        post_title: &str,
        post_body: &str,
        post_slug: &str,
        connection: &mut SqliteConnection,
    ) -> Post;

    fn update_post(
        &self,
        post_id: i32,
        post_title: &str,
        post_body: &str,
        connection: &mut SqliteConnection,
    ) -> Post;

    fn get_posts(&self, connection: &mut SqliteConnection) -> Vec<Post>;

    fn delete_post(&self, post_id: i32, connection: &mut SqliteConnection) -> bool;

    fn get_post_by_id(&self, post_id: i32, connection: &mut SqliteConnection) -> Option<Post>;
}

// The implementation of the trait
pub struct SqlitePostRepository;

impl PostRepository for SqlitePostRepository {
    fn create_post(
        &self,
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

    fn update_post(
        &self,
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
        let post = self.get_post_by_id(post_id, connection);
        post.unwrap()
    }

    fn get_posts(&self, connection: &mut SqliteConnection) -> Vec<Post> {
        use crate::schema::posts::dsl::posts;

        let results = posts
            .select(Post::as_select())
            .load(connection)
            .expect("Error loading posts");

        results
    }

    fn delete_post(&self, post_id: i32, connection: &mut SqliteConnection) -> bool {
        use crate::schema::posts::dsl::{id, posts};

        diesel::delete(posts.filter(id.eq(post_id)))
            .execute(connection)
            .expect("Error deleting the post")
            > 0
    }

    fn get_post_by_id(&self, post_id: i32, connection: &mut SqliteConnection) -> Option<Post> {
        use crate::schema::posts::dsl::{id, posts};

        posts
            .filter(id.eq(post_id))
            .first(connection)
            .optional()
            .expect("Error fetching the post")
    }
}

// Factory function to create a new repository
pub fn new_post_repository() -> impl PostRepository {
    SqlitePostRepository
}
