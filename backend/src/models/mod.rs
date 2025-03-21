use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub role: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ACHDetails {
    pub account_number: String,
    pub routing_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct BankTransferDetails {
    pub bank_name: String,
    pub account_number: String,
}
