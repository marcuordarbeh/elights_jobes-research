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

#[post("/process_card")]
async fn process_card(req: web::Json<CardDetails>) -> HttpResponse {
    match payment_service::process_card(&req.card_number, &req.expiry_date, &req.cvv).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/generate_ach")]
async fn generate_ach(req: web::Json<ACHDetails>) -> HttpResponse {
    match payment_service::generate_ach(&req.account_number, &req.routing_number).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/receive_bank_transfer")]
async fn receive_bank_transfer() -> HttpResponse {
    match payment_service::receive_bank_transfer().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/convert_to_crypto")]
async fn convert_to_crypto() -> HttpResponse {
    match payment_service::convert_to_crypto().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}