use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use crate::services::payment_service;

#[derive(Deserialize)]
struct CardDetails {
    card_number: String,
    expiry_date: String,
    cvv: String,
}

#[derive(Deserialize)]
struct ACHDetails {
    account_number: String,
    routing_number: String,
}

#[derive(Deserialize)]
struct CryptoConversionRequest {
    amount: f64,
}

#[post("/process_card")]
pub async fn process_card(req: web::Json<CardDetails>) -> HttpResponse {
    match payment_service::process_card(&req.card_number, &req.expiry_date, &req.cvv).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[post("/generate_ach")]
pub async fn generate_ach(
    req: web::Json<ACHDetails>,
    db_pool: web::Data<sqlx::PgPool>,
) -> HttpResponse {
    match payment_service::generate_ach(&db_pool, &req.account_number, &req.routing_number).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[post("/wire_transfer")]
pub async fn wire_transfer(
    db_pool: web::Data<sqlx::PgPool>,
) -> HttpResponse {
    match payment_service::receive_bank_transfer(&db_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[post("/convert_to_crypto")]
pub async fn convert_to_crypto(
    req: web::Json<CryptoConversionRequest>,
    redis_conn: web::Data<redis::aio::ConnectionManager>,
) -> HttpResponse {
    match payment_service::convert_to_crypto(&redis_conn, req.amount).await {
        Ok(wallet_address) => HttpResponse::Ok().body(wallet_address),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}
