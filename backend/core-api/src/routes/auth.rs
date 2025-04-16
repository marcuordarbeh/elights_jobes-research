// backend/core-api/src/routes/auth.rs
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/auth/login").route(web::post().to(super::handlers::auth::login)))
       .service(web::resource("/auth/register").route(web::post().to(super::handlers::auth::register)));
}

