// /home/inno/elights_jobes-research/backend/core-api/src/lib.rs

// Define modules comprising the core API application
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod middlewares;
pub mod models; // API-specific request/response models
pub mod routes;
pub mod services; // API-specific services (e.g., external API clients)
pub mod utils;

// Re-export key types for potential external use (e.g., integration testing)
pub use error::ApiError;
pub use config::AppConfig;
pub use db::{DbPool, init_db_pool}; // Export pool type and initializer
pub use crate::routes::configure_routes; // Export main route config function

// Include schema for Diesel access throughout the crate
// Assuming schema.rs is generated in `database/` and symlinked/copied to `src/schema.rs`
pub mod schema {
    // If schema.rs is in ../database/, adjust include path or use build script
    include!("../../database/schema.rs"); // Adjust relative path if needed
}