// mod.rs - Registers API routes.
use actix_web::web;

mod payment_controller;
// Note: All auth controllers are removed.

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(payment_controller::process_card)
            .service(payment_controller::generate_ach)
            .service(payment_controller::receive_bank_transfer)
            .service(payment_controller::convert_to_crypto),
    );
}
