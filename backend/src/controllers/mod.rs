use actix_web::web;

mod auth_controller;
mod payment_controller;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(auth_controller::register)
            .service(auth_controller::login)
            .service(payment_controller::process_card)
            .service(payment_controller::generate_ach)
            .service(payment_controller::receive_bank_transfer)
            .service(payment_controller::convert_to_crypto),
    );
}