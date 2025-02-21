use rand::Rng;
use stripe::{Client, PaymentIntent};
use std::env;
use crate::utils::crypto;
use crate::repositories::payment_repository;

pub async fn process_card(card_number: &str, expiry_date: &str, cvv: &str) -> Result<(), String> {
    let client = Client::new(env::var("STRIPE_SECRET_KEY").unwrap());
    let intent = PaymentIntent::create(&client, stripe::CreatePaymentIntent {
        amount: 1000, // amount in cents
        currency: "usd",
        payment_method_types: &["card"],
        ..Default::default()
    }).map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn generate_ach(account_number: &str, routing_number: &str) -> Result<(), String> {
    let ach_details = format!("{}{}", account_number, routing_number);
    payment_repository::save_ach_details(&ach_details).await.map_err(|e| e.to_string())
}

pub async fn receive_bank_transfer() -> Result<(), String> {
    let bank_name = generate_random_bank_name();
    let account_number = generate_random_account_number();
    payment_repository::save_bank_transfer_details(&bank_name, &account_number).await.map_err(|e| e.to_string())
}

pub async fn convert_to_crypto() -> Result<(), String> {
    crypto::convert_to_monero().await.map_err(|e| e.to_string())
}

fn generate_random_bank_name() -> String {
    let banks = vec!["Bank of America", "Chase", "Wells Fargo", "Citibank"];
    let mut rng = rand::thread_rng();
    banks[rng.gen_range(0..banks.len())].to_string()
}

fn generate_random_account_number() -> String {
    let mut rng = rand::thread_rng();
    rng.gen_range(1000000000..9999999999).to_string()
}