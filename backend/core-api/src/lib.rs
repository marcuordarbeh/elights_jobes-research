// /home/inno/elights_jobes-research/backend/core-api/src/lib.rs

// This file makes the crate usable as a library if needed,
// for example, in integration tests.

pub mod config;
pub mod error;
pub mod handlers;
pub mod middlewares;
pub mod models;
pub mod routes;
pub mod security;
pub mod services;
pub mod utils;

// Re-export key types or functions if needed
pub use error::ApiError;