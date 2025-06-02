use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PostSimplified {
    pub title: String,
    pub body: String,
}

use super::schema::posts;

#[derive(Insertable, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}

#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub body: String,
    pub slug: String,
}

#[derive(Deserialize)]
pub struct UpdatePostRequest {
    pub title: String,
    pub body: String,
}
