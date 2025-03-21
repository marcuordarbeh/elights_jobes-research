use rand::Rng;
use stripe::{Client, PaymentIntent, CreatePaymentIntent};
use std::env;
use crate::repositories::payment_repository;
use crate::utils::crypto;
use sqlx::PgPool;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;

pub async fn process_card(
    card_number: &str,
    expiry_date: &str,
    cvv: &str,
) -> Result<(), String> {
    let client = Client::new(
        env::var("STRIPE_SECRET_KEY")
            .expect("STRIPE_SECRET_KEY must be set"),
    );
    let _intent = PaymentIntent::create(
        &client,
        CreatePaymentIntent {
            amount: 1000, // in cents
            currency: "usd",
            payment_method_types: &["card"],
            ..Default::default()
        },
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn generate_ach(
    pool: &PgPool,
    account_number: &str,
    routing_number: &str,
) -> Result<(), String> {
    let ach_details = format!("ACH:{}-{}", account_number, routing_number);
    payment_repository::save_ach_details(pool, &ach_details)
        .await
        .map_err(|e| e.to_string())
}

pub async fn receive_bank_transfer(pool: &PgPool) -> Result<(), String> {
    let bank_name = generate_random_bank_name();
    let account_number = generate_random_account_number();
    payment_repository::save_bank_transfer_details(pool, &bank_name, &account_number)
        .await
        .map_err(|e| e.to_string())
}

pub async fn convert_to_crypto(
    redis_conn: &ConnectionManager,
    amount: f64,
) -> Result<String, String> {
    // Call external API via Tor proxy to convert USD to Monero
    let wallet_address =
        crypto::convert_to_monero(amount).await.map_err(|e| e.to_string())?;
    // Cache the wallet address in Redis for 5 minutes
    let mut conn = redis_conn.clone();
    let cache_key = format!("crypto_wallet:{}", amount);
    let _: () = conn
        .set_ex(&cache_key, &wallet_address, 300)
        .await
        .map_err(|e| e.to_string())?;
    Ok(wallet_address)
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
