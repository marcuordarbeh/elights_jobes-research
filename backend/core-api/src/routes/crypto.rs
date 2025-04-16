use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/crypto/convert")
            .route(web::post().to(super::super::handlers::crypto::convert))
    );
}
