use crate::domain::{Post, PostRepository};
use crate::infrastructure::database::{DbPool, run_with_connection};
use crate::infrastructure::persistence::models::{PostModel, NewPostModel};
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;
use diesel::QueryableByName;
use std::sync::Arc;

// Helper struct for getting the last inserted ID
#[derive(QueryableByName)]
struct LastId {
    #[diesel(sql_type = Integer)]
    last_insert_rowid: i32,
}

/// SQLite implementation of the PostRepository port
pub struct SqlitePostRepository {
    pool: Arc<DbPool>,
}

impl SqlitePostRepository {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostRepository for SqlitePostRepository {
    async fn find_all(&self) -> Result<Vec<Post>, String> {
        let pool = Arc::clone(&self.pool);
        
        tokio::task::spawn_blocking(move || {
            run_with_connection(&pool, |conn| {
                use crate::schema::posts::dsl::*;
                
                posts
                    .select(PostModel::as_select())
                    .load(conn)
                    .map(|models: Vec<PostModel>| {
                        models.into_iter().map(Post::from).collect()
                    })
                    .map_err(|e| format!("Database error: {}", e))
            })
        })
        .await
        .map_err(|e| format!("Task error: {}", e))?
    }

    async fn find_by_id(&self, post_id: i32) -> Result<Option<Post>, String> {
        let pool = Arc::clone(&self.pool);
        
        tokio::task::spawn_blocking(move || {
            run_with_connection(&pool, |conn| {
                use crate::schema::posts::dsl::*;
                
                posts
                    .filter(id.eq(post_id))
                    .first::<PostModel>(conn)
                    .optional()
                    .map(|opt| opt.map(Post::from))
                    .map_err(|e| format!("Database error: {}", e))
            })
        })
        .await
        .map_err(|e| format!("Task error: {}", e))?
    }

    async fn save(&self, post: Post) -> Result<Post, String> {
        let pool = Arc::clone(&self.pool);
        
        tokio::task::spawn_blocking(move || {
            run_with_connection(&pool, |conn| {
                use crate::schema::posts::dsl::*;

                let new_post = NewPostModel::from(&post);

                // Use transaction for atomic operation
                conn.transaction(|conn| {
                    // Insert the post
                    diesel::insert_into(posts)
                        .values(&new_post)
                        .execute(conn)?;

                    // Get the last inserted row ID
                    let result = sql_query("SELECT last_insert_rowid() as last_insert_rowid")
                        .get_result::<LastId>(conn)?;

                    let last_id = result.last_insert_rowid;

                    // Fetch the newly created post
                    let created_post: PostModel = posts
                        .filter(id.eq(last_id))
                        .first(conn)?;

                    Ok(Post::from(created_post))
                })
                .map_err(|e: diesel::result::Error| format!("Database error: {}", e))
            })
        })
        .await
        .map_err(|e| format!("Task error: {}", e))?
    }

    async fn update(&self, post_id: i32, post: Post) -> Result<Option<Post>, String> {
        let pool = Arc::clone(&self.pool);
        
        tokio::task::spawn_blocking(move || {
            run_with_connection(&pool, |conn| {
                use crate::schema::posts::dsl::*;

                // First update the post
                let rows_affected = diesel::update(posts)
                    .filter(id.eq(post_id))
                    .set((title.eq(&post.title), body.eq(&post.body)))
                    .execute(conn)
                    .map_err(|e| format!("Database error: {}", e))?;

                if rows_affected == 0 {
                    return Ok(None);
                }

                // Then fetch the updated post
                posts
                    .filter(id.eq(post_id))
                    .first::<PostModel>(conn)
                    .optional()
                    .map(|opt| opt.map(Post::from))
                    .map_err(|e| format!("Database error: {}", e))
            })
        })
        .await
        .map_err(|e| format!("Task error: {}", e))?
    }

    async fn delete(&self, post_id: i32) -> Result<bool, String> {
        let pool = Arc::clone(&self.pool);
        
        tokio::task::spawn_blocking(move || {
            run_with_connection(&pool, |conn| {
                use crate::schema::posts::dsl::*;

                diesel::delete(posts.filter(id.eq(post_id)))
                    .execute(conn)
                    .map(|rows_affected| rows_affected > 0)
                    .map_err(|e| format!("Database error: {}", e))
            })
        })
        .await
        .map_err(|e| format!("Task error: {}", e))?
    }
}
