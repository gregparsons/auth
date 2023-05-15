//! sqlx_pool.rs

use sqlx::{PgPool, Pool, Postgres};

/// start up the database pool
pub async fn create_sqlx_pg_pool() -> Pool<Postgres> {
    // DB connect; get some data; currently just one stock for the last three days
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in environment");
    let pool = PgPool::connect(&db_url).await.expect("[main] failed to connect to postgres");
    pool
}
