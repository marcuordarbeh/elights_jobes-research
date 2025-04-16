use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/payments/initiate")
            .route(web::post().to(super::super::handlers::payments::initiate_payment))
    )
    .service(
        web::resource("/payments/status")
            .route(web::get().to(super::super::handlers::payments::payment_status))
    );
}

// use actix_web::{post, web, HttpResponse};
// use serde::Deserialize;
// use crate::services::payment_service;

// #[derive(Deserialize)]
// struct CardDetails {
//     card_number: String,
//     expiry_date: String,
//     cvv: String,
// }

// #[allow(dead_code)]
// #[derive(Deserialize)]
// struct ACHInput {
//     account_number: String,
//     routing_number: String,
// }

// #[post("/process_card")]
// pub async fn process_card(req: web::Json<CardDetails>) -> HttpResponse {
//     match payment_service::process_card(&req.card_number, &req.expiry_date, &req.cvv).await {
//         Ok(result) => HttpResponse::Ok().json(result),
//         Err(err) => HttpResponse::InternalServerError().body(err),
//     }
// }

// #[post("/generate_ach")]
// pub async fn generate_ach(db_pool: web::Data<sqlx::PgPool>) -> HttpResponse {
//     match payment_service::generate_ach(&db_pool).await {
//         Ok(_) => HttpResponse::Ok().body("ACH details generated successfully"),
//         Err(err) => HttpResponse::InternalServerError().body(err),
//     }
// }

// #[post("/receive_bank_transfer")]
// pub async fn receive_bank_transfer(db_pool: web::Data<sqlx::PgPool>) -> HttpResponse {
//     match payment_service::receive_bank_transfer(&db_pool).await {
//         Ok(_) => HttpResponse::Ok().body("Wire transfer details generated successfully"),
//         Err(err) => HttpResponse::InternalServerError().body(err),
//     }
// }

// #[post("/convert_to_crypto")]
// pub async fn convert_to_crypto() -> HttpResponse {
//     match payment_service::convert_to_crypto().await {
//         Ok(_) => HttpResponse::Ok().body("Conversion to crypto completed successfully"),
//         Err(err) => HttpResponse::InternalServerError().body(err),
//     }
// }
