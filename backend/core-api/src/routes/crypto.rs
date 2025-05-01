// /home/inno/elights_jobes-research/backend/core-api/src/routes/crypto.rs
use actix_web::web;
use crate::handlers::crypto::{
    convert_crypto, get_wallet_balance, initiate_crypto_withdrawal // Import handlers
};

/// Configures cryptocurrency related routes.
pub fn configure_crypto_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/crypto") // Scope for crypto endpoints
            .route("/convert", web::post().to(convert_crypto)) // Renamed handler
            .route("/wallets/{wallet_id}/balance", web::get().to(get_wallet_balance)) // Example
            .route("/withdrawals", web::post().to(initiate_crypto_withdrawal)) // Example
            // Add routes for getting deposit addresses, transaction history, etc.
    );
}