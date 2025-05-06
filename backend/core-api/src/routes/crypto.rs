// /home/inno/elights_jobes-research/backend/core-api/src/routes/crypto.rs
use actix_web::web;
use crate::handlers::crypto::{
    get_conversion_quote, // Renamed from convert_crypto
    execute_crypto_conversion, // New handler for execution
    get_wallet_balance,
    initiate_crypto_withdrawal,
    get_deposit_address,
    handle_crypto_webhook, // For BTCPay/Monero notifications
};
use crate::middlewares::auth_guard::AuthGuard; // Requires authentication

/// Configures cryptocurrency related routes: `/api/v1/crypto/...`
pub fn configure_crypto_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/crypto")
            // Conversion endpoints
            .route("/quote", web::post().to(get_conversion_quote).wrap(AuthGuard)) // Get quote
            .route("/convert", web::post().to(execute_crypto_conversion).wrap(AuthGuard)) // Execute conversion

            // Wallet management endpoints
            .route("/wallets/{wallet_id}/balance", web::get().to(get_wallet_balance).wrap(AuthGuard))
            .route("/wallets/{wallet_type}/address", web::get().to(get_deposit_address).wrap(AuthGuard)) // Get deposit address for type

            // Withdrawal endpoint
            .route("/withdrawals", web::post().to(initiate_crypto_withdrawal).wrap(AuthGuard))

            // Webhook for incoming crypto events (e.g., BTCPay invoice updates)
            .route("/webhook/{provider}", web::post().to(handle_crypto_webhook)) // provider=btcpay, monero ?

            // TODO: Add routes for crypto transaction history
            // .route("/history", web::get().to(list_crypto_history).wrap(AuthGuard))
    );
}