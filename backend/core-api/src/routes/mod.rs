// /home/inno/elights_jobes-research/backend/core-api/src/routes/mod.rs
use actix_web::web;

// Import route module configurations
mod auth;
mod crypto;
mod ft_integration; // Financial Times API integration routes
mod payments;
// mod health; // Optional: Add a health check route

/// Configures all API routes under the `/api/v1` scope.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1") // Base path for API version 1
            // .configure(health::configure_health_routes) // Example health check
            .configure(auth::configure_auth_routes)
            .configure(payments::configure_payment_routes)
            .configure(crypto::configure_crypto_routes)
            .configure(ft_integration::configure_ft_routes)
            // Add configurations for other route modules here
            // e.g., user profile management, admin endpoints
    );
}

// Optional: Add a simple root endpoint for basic check
// pub async fn index() -> impl actix_web::Responder {
//     actix_web::HttpResponse::Ok().body("Elights Core API Service")
// }
// In main.rs: App::new().route("/", web::get().to(index)) ...