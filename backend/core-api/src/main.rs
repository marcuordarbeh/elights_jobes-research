// /home/inno/elights_jobes-research/backend/core-api/src/main.rs

use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use sqlx::postgres::{PgPool, PgPoolOptions}; // Using SQLx example
use crate::config::{database, bank_server::BankServerConfig, ft_asset::FtAssetConfig};
use crate::middlewares::logger::RequestLogger; // Use custom logger
use crate::error::ApiError;
use crate::security::tls::load_tls_config;

mod config;
mod handlers;
mod middlewares;
mod models; // Optional: Define API-specific request/response models here
mod routes;
mod security; // TLS loading logic
mod services; // API specific services (if any, mostly calls domain)
mod utils;
mod error; // API error types

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load configurations
    let db_url = database::get_database_url();
    let bank_server_config = BankServerConfig::from_env().map_err(log_config_error)?;
    let _ft_asset_config = FtAssetConfig::from_env().map_err(log_config_error)?; // Load but unused directly here

    let server_address = format!("{}:{}", bank_server_config.host, bank_server_config.port);
    let api_address = env::var("API_BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string()); // Separate API bind addr

    // Establish database connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(10) // Configure pool size
        .connect(&db_url)
        .await
        .map_err(|e| {
            log::error!("Failed to create database pool: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;

    log::info!("Database pool initialized successfully.");

    // Load TLS configuration
    let tls_config = load_tls_config(
        &bank_server_config.server_cert,
        &bank_server_config.server_key
    ).map_err(log_config_error)?;

    log::info!("TLS configuration loaded successfully.");
    log::info!("🚀 Starting Core API Server at http://{}", api_address);
    log::info!("   Bank Server (TLS) endpoint configured for: https://{}", server_address); // Log TLS info

    // Start Actix HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone())) // Share db pool
            // TODO: Add other app_data like API keys, clients etc. if needed
            // .app_data(web::Data::new(ChaseClient::new().unwrap())) // Example
            .wrap(Logger::default()) // Default Actix logger
            .wrap(RequestLogger::default()) // Your custom logger middleware
            // TODO: Configure CORS if needed
            // TODO: Add Authentication Middleware if needed
            // TODO: Add IP Whitelist Middleware if configured
            .configure(routes::configure_routes) // Register all routes
    })
    .bind(&api_address)? // Bind non-TLS endpoint for API
    // Optional: Bind the TLS endpoint if this service handles direct bank connections over TLS
    // .bind_rustls(&server_address, tls_config)?
    .run()
    .await
}

// Helper to convert DomainError to io::Error for main
fn log_config_error<E: std::fmt::Display>(e: E) -> std::io::Error {
     log::error!("Configuration Error: {}", e);
     std::io::Error::new(std::io::ErrorKind::Other, format!("Config error: {}", e))
}