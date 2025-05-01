// /home/inno/elights_jobes-research/backend/core-api/src/routes/conversion.rs
use actix_web::web;
use crate::handlers::conversion::{get_conversion_rate, perform_currency_conversion}; // Import handlers

/// Configures currency conversion related routes.
pub fn configure_conversion_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/conversions") // Scope for conversion endpoints
            .route("/rate", web::get().to(get_conversion_rate)) // Get rate only
            .route("/execute", web::post().to(perform_currency_conversion)) // Perform conversion
    );
}