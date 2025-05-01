// /home/inno/elights_jobes-research/backend/core-api/src/routes/mod.rs
use actix_web::web;

// Import route modules
pub mod auth;
pub mod payments;
pub mod crypto;
pub mod conversion;
// Add other route modules if needed

/// Configures all API routes under the `/api/v1` scope.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1") // Use versioned API path
            .configure(auth::configure_auth_routes)
            .configure(payments::configure_payment_routes)
            .configure(crypto::configure_crypto_routes)
            .configure(conversion::configure_conversion_routes)
            // Add configurations for other route modules here
    );
}