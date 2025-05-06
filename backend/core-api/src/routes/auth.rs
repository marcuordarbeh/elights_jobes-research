// /home/inno/elights_jobes-research/backend/core-api/src/routes/auth.rs
use actix_web::web;
use crate::handlers::auth::{login, register}; // Import handlers

/// Configures authentication related routes: `/api/v1/auth/...`
pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            // TODO: Add routes for token refresh, password reset, email verification etc.
            // .route("/refresh", web::post().to(refresh_token))
            // .route("/logout", web::post().to(logout)) // Requires auth middleware
    );
}