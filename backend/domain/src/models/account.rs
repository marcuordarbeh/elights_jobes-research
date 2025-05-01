// /home/inno/elights_jobes-research/backend/domain/src/models/account.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal; // Use Decimal for financial values

// use crate::schema::core_schema::accounts; // For Diesel
// use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AccountType {
    FiatBank,
    CryptoXmr,
    CryptoBtc,
    // Add other types as needed
}

// #[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable)] // Add Diesel traits
// #[diesel(table_name = accounts)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Account {
    pub id: i32, // Serial primary key
    pub owner_username: String, // Foreign key to users table
    pub account_identifier: String, // Unique identifier (e.g., IBAN + BIC, account number, crypto address)
    pub account_type: AccountType,
    pub currency: String, // ISO 4217 or Ticker
    #[serde(with = "rust_decimal::serde::str")] // Serialize Decimal as string for precision
    pub balance: Decimal,
    pub bank_name: Option<String>,
    pub routing_number: Option<String>,
    pub iban: Option<String>,
    pub bic_swift: Option<String>,
    pub crypto_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Add NewAccount struct for Diesel if needed
// #[derive(Insertable)]
// #[diesel(table_name = accounts)]
// pub struct NewAccount { ... }