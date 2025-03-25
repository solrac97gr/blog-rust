pub mod models;
pub mod posts;
pub mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .unwrap_or_else(|_| panic!("Error creating connection pool"))
}

pub fn run_with_connection<F, T>(pool: &Pool<ConnectionManager<SqliteConnection>>, f: F) -> T
where
    F: FnOnce(&mut SqliteConnection) -> T,
{
    let mut conn = pool.get().expect("Failed to get connection from pool");
    f(&mut conn)
}
