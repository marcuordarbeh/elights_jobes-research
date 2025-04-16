use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CurrencyConversionRequest {
    amount: f64,
    from: String,
    to: String,
}

#[derive(Debug, Serialize)]
pub struct CurrencyConversionResponse {
    converted_amount: f64,
    conversion_rate: f64,
}

pub async fn currency_conversion(info: web::Json<CurrencyConversionRequest>) -> HttpResponse {
    // Add real conversion logic here (e.g., call an external FX rates API)
    println!("Converting {} {} to {}", info.amount, info.from, info.to);
    HttpResponse::Ok().json(CurrencyConversionResponse {
        converted_amount: info.amount * 1.1, // Dummy conversion rate
        conversion_rate: 1.1,
    })
}
