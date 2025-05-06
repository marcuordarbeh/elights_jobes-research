// /home/inno/elights_jobes-research/backend/core-api/src/routes/payments.rs
use actix_web::web;
use crate::handlers::payments::{initiate_payment, get_payment_status, handle_payment_webhook};
use crate::middlewares::auth_guard::AuthGuard; // Import auth middleware

/// Configures payment related routes: `/api/v1/payments/...`
pub fn configure_payment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/payments")
            // Endpoint to initiate various payment types (needs auth)
            .route("/initiate", web::post().to(initiate_payment).wrap(AuthGuard))
            // Endpoint to get status of a specific transaction (needs auth)
            .route("/status/{transaction_id}", web::get().to(get_payment_status).wrap(AuthGuard))
            // Public endpoint for receiving payment status updates from external providers
            .route("/webhook/{provider}", web::post().to(handle_payment_webhook)) // e.g., provider=stripe, btcpay
            // TODO: Add routes for listing user's payment history (with pagination)
            // .route("/history", web::get().to(list_payment_history).wrap(AuthGuard))
    );
}