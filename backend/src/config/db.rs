use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn init_pool(database_url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("Failed to create PostgreSQL pool.")
}
