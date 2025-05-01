// /home/inno/elights_jobes-research/backend/core-api/src/routes/auth.rs
use actix_web::web;
use crate::handlers::auth::{login, register}; // Import handlers

/// Configures authentication related routes.
pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth") // Scope for auth endpoints
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
            // Add other auth routes like /refresh_token, /logout etc.
    );
}