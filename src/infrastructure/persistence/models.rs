use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Database model for posts (used by Diesel ORM)
#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PostModel {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

/// Model for inserting new posts
#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPostModel<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}

impl From<PostModel> for crate::domain::Post {
    fn from(model: PostModel) -> Self {
        crate::domain::Post::with_id(model.id, model.title, model.slug, model.body)
    }
}

impl<'a> From<&'a crate::domain::Post> for NewPostModel<'a> {
    fn from(post: &'a crate::domain::Post) -> Self {
        NewPostModel {
            title: &post.title,
            slug: &post.slug,
            body: &post.body,
        }
    }
}
