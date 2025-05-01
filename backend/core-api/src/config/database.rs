// /home/inno/elights_jobes-research/backend/core-api/src/config/database.rs
use dotenv::dotenv;
use std::env;
use crate::error::ApiError; // Use ApiError for config issues

/// Retrieves the database connection URL from environment variables.
pub fn get_database_url() -> Result<String, ApiError> {
    dotenv().ok(); // Load .env file if present
    env::var("DATABASE_URL")
        .map_err(|_| ApiError::ConfigurationError("DATABASE_URL environment variable not set".to_string()))
}

// Example function to initialize a DB pool (using SQLx)
// pub async fn init_pool(database_url: &str) -> Result<sqlx::PgPool, ApiError> {
//     sqlx::postgres::PgPoolOptions::new()
//         .max_connections(10) // Example pool size
//         .connect(database_url)
//         .await
//         .map_err(|e| ApiError::DatabaseError(format!("Failed to create database pool: {}", e)))
// }