use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CryptoConversionRequest {
    amount: f64,
    from: String,
    to: String,
}

#[derive(Debug, Serialize)]
pub struct CryptoConversionResponse {
    converted_amount: f64,
    conversion_rate: f64,
}

pub async fn convert(info: web::Json<CryptoConversionRequest>) -> HttpResponse {
    // Insert your integration logic with BTCPayServer and Gopenmonero here
    println!(
        "Converting {} {} to {}",
        info.amount, info.from, info.to
    );
    HttpResponse::Ok().json(CryptoConversionResponse {
        converted_amount: info.amount * 0.95, // Dummy conversion
        conversion_rate: 0.95,
    })
}
