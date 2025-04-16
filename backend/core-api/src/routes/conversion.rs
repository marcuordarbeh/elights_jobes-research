use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/conversion/currency")
            .route(web::post().to(super::super::handlers::conversion::currency_conversion))
    );
}
