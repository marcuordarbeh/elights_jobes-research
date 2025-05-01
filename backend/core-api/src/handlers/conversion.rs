// /home/inno/elights_jobes-research/backend/core-api/src/handlers/conversion.rs
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::error::ApiError;
use rust_decimal::Decimal;

#[derive(Debug, Deserialize)]
pub struct CurrencyConversionRequest {
    amount: Decimal, // Use Decimal
    from_currency: String, // ISO 4217
    to_currency: String,   // ISO 4217
}

#[derive(Debug, Serialize)]
pub struct CurrencyConversionResponse {
    original_amount: String,
    converted_amount: String,
    from_currency: String,
    to_currency: String,
    rate: String,
}

/// Gets the current conversion rate between two currencies.
pub async fn get_conversion_rate(
    // TODO: Inject rate service client
    query: web::Query<ConversionRateQuery>,
) -> Result<impl Responder, ApiError> {
    log::info!("Fetching conversion rate: {} -> {}", query.from, query.to);
    // --- Placeholder Logic ---
    // 1. Validate currency codes
    // 2. Call external FX rate service API
    let dummy_rate = Decimal::new(110, 2); // Example rate 1.10

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "from": query.from,
        "to": query.to,
        "rate": dummy_rate.to_string(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })))
    // --- End Placeholder Logic ---
}


/// Performs a currency conversion.
pub async fn perform_currency_conversion(
    // TODO: Inject rate service client
    info: web::Json<CurrencyConversionRequest>,
) -> Result<impl Responder, ApiError> {
     log::info!("Performing conversion: {} {} -> {}", info.amount, info.from_currency, info.to_currency);

    if info.amount <= Decimal::ZERO {
        return Err(ApiError::ValidationError("Amount must be positive".to_string()));
    }
     // --- Placeholder Logic ---
     // 1. Validate currency codes
     // 2. Fetch current conversion rate
     let dummy_rate = Decimal::new(110, 2); // Example rate 1.10

     // 3. Calculate converted amount
     let converted_amount = info.amount * dummy_rate; // Simplified calculation

     // TODO: Handle potential fees, spreads, precision for real conversions

     Ok(HttpResponse::Ok().json(CurrencyConversionResponse {
        original_amount: info.amount.to_string(),
        converted_amount: converted_amount.to_string(),
        from_currency: info.from_currency.clone(),
        to_currency: info.to_currency.clone(),
        rate: dummy_rate.to_string(),
    }))
     // --- End Placeholder Logic ---
}

#[derive(Debug, Deserialize)]
pub struct ConversionRateQuery {
    from: String,
    to: String,
}