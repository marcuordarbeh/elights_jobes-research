// /home/inno/elights_jobes-research/backend/core-api/src/routes/ft_integration.rs
use actix_web::web;
use crate::handlers::ft_integration::{handle_ft_notification, get_ft_content};
// Decide on authentication for these endpoints. Webhook might be public but verified.
// use crate::middlewares::auth_guard::AuthGuard;

/// Configures Financial Times API integration routes: `/api/v1/ft/...`
pub fn configure_ft_routes(cfg: &mut web::ServiceConfig) {
     cfg.service(
        web::scope("/ft")
            // Public endpoint to receive push notifications from FT API
            // Needs signature verification in the handler
            .route("/notifications/push", web::post().to(handle_ft_notification))

            // Example endpoint to fetch content (might need auth depending on use case)
            // Requires content UUID from notification or other source
            .route("/content/{content_uuid}", web::get().to(get_ft_content)) // .wrap(AuthGuard) ?
    );
}