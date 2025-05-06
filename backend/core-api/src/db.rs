// /home/inno/elights_jobes-research/backend/core-api/src/db.rs
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};
use crate::error::ApiError; // Use API error type

// Type alias for the Diesel PostgreSQL connection pool
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Initializes the database connection pool.
pub fn init_db_pool(database_url: &str) -> Result<DbPool, ApiError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .map_err(|e| {
             log::error!("Failed to create database pool: {}", e);
             ApiError::ConfigurationError(format!("Database pool initialization failed: {}", e))
        })
}

/// Helper function to get a connection from the pool.
pub fn get_db_conn(pool: &web::Data<DbPool>) -> Result<DbPooledConnection, ApiError> {
    pool.get().map_err(|e| {
        log::error!("Failed to get DB connection from pool: {}", e);
        ApiError::DbPoolError(e) // Use specific pool error variant
    })
}

use actix_web::web; // Required for web::Data