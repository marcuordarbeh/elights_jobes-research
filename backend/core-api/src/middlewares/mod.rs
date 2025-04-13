// mod.rs - Model definitions. We remove any authentication specifics.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: String,
}

// Claims model – kept here if needed later.
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub role: String,
    pub exp: usize,
}

// Model for ACH details (for saving to DB)
#[derive(Serialize, Deserialize)]
pub struct ACHDetails {
    pub account_number: String,
    pub routing_number: String,
}

// Model for bank transfer details
#[derive(Serialize, Deserialize)]
pub struct BankTransferDetails {
    pub bank_name: String,
    pub account_number: String,
}
