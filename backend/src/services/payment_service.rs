// payment_service.rs - Implements card processing, ACH and wire generation, and fiat-to-crypto conversion.
use rand::Rng;
use sqlx::PgPool;
use crate::repositories::payment_repository;
use crate::utils::crypto;

// Process card details by simulating a transaction and converting fiat to Monero.
pub async fn process_card(card_number: &str, expiry_date: &str, cvv: &str) -> Result<serde_json::Value, String> {
    let amount = 1000; // Simulated amount (in cents)
    let conversion_result = crypto::convert_to_monero(amount).await.map_err(|e| e.to_string())?;
    Ok(serde_json::json!({
        "card": {
            "number": card_number,
            "expiry": expiry_date,
            "cvv": cvv,
            "processed_amount": amount
        },
        "monero_conversion": conversion_result,
        "status": "processed"
    }))
}

// Generate random ACH details (Nacha style) and save to database.
pub async fn generate_ach(pool: &PgPool) -> Result<(), String> {
    let routing_number = (100_000_000_i64 + rand::thread_rng().gen_range(0..900_000_000_i64)).to_string();
    let account_number = (1_000_000_000_i64 + rand::thread_rng().gen_range(0..9_000_000_000_i64)).to_string();
    let ach_details = format!("ACH:{}-{}", account_number, routing_number);
    payment_repository::save_ach_details(pool, &ach_details)
        .await
        .map_err(|e| e.to_string())
}

// Generate random wire transfer details and save to database.
pub async fn receive_bank_transfer(pool: &PgPool) -> Result<(), String> {
    let banks = vec!["Bank of America", "Chase", "Wells Fargo", "Citibank"];
    let bank_name = banks[rand::thread_rng().gen_range(0..banks.len())].to_string();
    let account_number = (1_000_000_000_i64 + rand::thread_rng().gen_range(0..9_000_000_000_i64)).to_string();
    payment_repository::save_bank_transfer_details(pool, &bank_name, &account_number)
        .await
        .map_err(|e| e.to_string())
}

// // Generate random ACH details (Nacha style) and save to database.
// pub async fn generate_ach(pool: &PgPool) -> Result<(), String> {
//     let routing_number = (100000000 + rand::thread_rng().gen_range(0..900000000)).to_string();
//     let account_number = (1000000000 + rand::thread_rng().gen_range(0..9000000000)).to_string();
//     let ach_details = format!("ACH:{}-{}", account_number, routing_number);
//     payment_repository::save_ach_details(pool, &ach_details)
//         .await
//         .map_err(|e| e.to_string())
// }

// // Generate random wire transfer details and save to database.
// pub async fn receive_bank_transfer(pool: &PgPool) -> Result<(), String> {
//     let banks = vec!["Bank of America", "Chase", "Wells Fargo", "Citibank"];
//     let bank_name = banks[rand::thread_rng().gen_range(0..banks.len())].to_string();
//     let account_number = (1000000000 + rand::thread_rng().gen_range(0..9000000000)).to_string();
//     payment_repository::save_bank_transfer_details(pool, &bank_name, &account_number)
//         .await
//         .map_err(|e| e.to_string())
// }

// Convert fiat to Monero using external API.
pub async fn convert_to_crypto() -> Result<serde_json::Value, String> {
    let conversion = crate::utils::crypto::convert_to_monero(250).await.map_err(|e| e.to_string())?;
    Ok(serde_json::json!({
        "status": "conversion successful",
        "result": conversion
    }))
}

// use rand::Rng;
// use sqlx::PgPool;
// use crate::repositories::payment_repository;
// use crate::utils::crypto;
// use std::env;

// // Simulate processing a card: here we just call our new simulated processing function.
// pub async fn process_card(card_number: &str, expiry_date: &str, cvv: &str) -> Result<serde_json::Value, String> {
//     // In a real integration, you'd securely process card details via a PCI‑compliant processor.
//     // For this demo, we simulate a card transaction then call fiat-to-Monero conversion.
//     let amount = 1000; // Simulated amount in USD cents
//     let conversion_result = crypto::convert_to_monero(amount).await.map_err(|e| e.to_string())?;
//     // Return a JSON object with simulated transaction info.
//     Ok(serde_json::json!({
//         "card": {
//             "number": card_number,
//             "expiry": expiry_date,
//             "cvv": cvv,
//             "processed_amount": amount
//         },
//         "monero_conversion": conversion_result,
//         "status": "processed"
//     }))
// }

// // Generate random ACH details and save them in the database
// pub async fn generate_ach(pool: &PgPool) -> Result<(), String> {
//     let routing_number = (100000000 + rand::thread_rng().gen_range(0..900000000)).to_string();
//     let account_number = (1000000000 + rand::thread_rng().gen_range(0..9000000000)).to_string();
//     let ach_details = format!("ACH:{}-{}", account_number, routing_number);
//     payment_repository::save_ach_details(pool, &ach_details).await.map_err(|e| e.to_string())
// }

// // Generate random wire transfer details and save them in the database
// pub async fn receive_bank_transfer(pool: &PgPool) -> Result<(), String> {
//     let banks = vec!["Bank of America", "Chase", "Wells Fargo", "Citibank"];
//     let bank_name = banks[rand::thread_rng().gen_range(0..banks.len())].to_string();
//     let account_number = (1000000000 + rand::thread_rng().gen_range(0..9000000000)).to_string();
//     payment_repository::save_bank_transfer_details(pool, &bank_name, &account_number)
//         .await
//         .map_err(|e| e.to_string())
// }

// // Convert fiat to crypto (Monero) using an external API
// pub async fn convert_to_crypto() -> Result<serde_json::Value, String> {
//     crypto::convert_to_monero().await.map_err(|e| e.to_string())?;
//     // For simplicity, we assume crypto conversion is performed and output logged.
//     Ok(serde_json::json!({"status": "conversion successful"}))
// }
