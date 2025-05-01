// /home/inno/elights_jobes-research/backend/core-api/src/routes/payments.rs
use actix_web::web;
use crate::handlers::payments::{initiate_payment, get_payment_status, handle_payment_webhook}; // Import handlers

/// Configures payment related routes.
pub fn configure_payment_routes(cfg: &mut web::ServiceConfig) {
     cfg.service(
        web::scope("/payments") // Scope for payment endpoints
            .route("/initiate", web::post().to(initiate_payment))
            .route("/{transaction_id}/status", web::get().to(get_payment_status)) // Get status by ID
            .route("/webhook", web::post().to(handle_payment_webhook)) // Example webhook endpoint
            // Add routes for specific payment types if needed (e.g., /ach, /wire)
    );
}