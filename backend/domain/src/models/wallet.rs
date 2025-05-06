// /home/inno/elights_jobes-research/backend/domain/src/models/wallet.rs
use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::{table, sql_types::{Uuid as DieselUuid, Varchar, Numeric as DieselNumeric, Nullable, Int4, Text, Timestamptz}};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rust_decimal::Decimal; // Use Decimal for financial values
use bigdecimal::BigDecimal; // Diesel uses BigDecimal for Numeric mapping

// Import the schema definition (similar issue as in user.rs)
// use schema::core_schema::wallets;
// TODO: Resolve schema path access. Using direct table reference for now.
table! {
    core_schema.wallets (wallet_id) {
        wallet_id -> DieselUuid,
        user_id -> DieselUuid,
        wallet_type -> Varchar,
        currency_code -> Varchar,
        balance -> DieselNumeric, // Maps to BigDecimal
        bank_name -> Nullable<Varchar>,
        account_number_hash -> Nullable<Text>,
        iban_hash -> Nullable<Text>,
        bic_swift -> Nullable<Varchar>,
        routing_number_hash -> Nullable<Text>,
        address -> Nullable<Varchar>,
        address_index -> Nullable<Int4>,
        status -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "core_schema.sql_types::WalletType"] // Assuming you create TYPE wallet_type in SQL
pub enum WalletType {
    FiatUsd,
    FiatEur,
    CryptoBtc,
    CryptoXmr,
    // Add other specific types
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "core_schema.sql_types::WalletStatus"] // Assuming you create TYPE wallet_status
pub enum WalletStatus {
    Active,
    Inactive,
    Suspended,
    Closed,
}

// Conversion between BigDecimal (Diesel) and Decimal (Application)
// Add this to a utils module or directly here if needed
fn bigdecimal_to_decimal(bd: BigDecimal) -> Decimal {
    Decimal::from_str_radix(&bd.to_string(), 10).unwrap_or_default()
    // TODO: Handle potential conversion errors more gracefully
}

fn decimal_to_bigdecimal(d: Decimal) -> BigDecimal {
     BigDecimal::from_str_radix(&d.to_string(), 10).unwrap_or_default()
     // TODO: Handle potential conversion errors more gracefully
}

/// Represents a user's wallet/account for a specific currency.
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable, Clone, PartialEq)]
#[diesel(table_name = wallets, primary_key(wallet_id))]
pub struct Wallet {
    pub wallet_id: Uuid,
    pub user_id: Uuid, // Foreign key to users table
    pub wallet_type: String, // Ideally map to WalletType enum if DB enum type is used
    pub currency_code: String, // ISO 4217 or Ticker
    #[diesel(deserialize_as = BigDecimal)] // Map DB Numeric to BigDecimal
    #[serde(with = "rust_decimal::serde::str")] // Serialize App's Decimal as string
    pub balance: Decimal, // Use Decimal in application logic
    pub bank_name: Option<String>,
    #[serde(skip_serializing)] // Never expose hashes
    pub account_number_hash: Option<String>,
    #[serde(skip_serializing)]
    pub iban_hash: Option<String>,
    pub bic_swift: Option<String>,
    #[serde(skip_serializing)]
    pub routing_number_hash: Option<String>,
    pub address: Option<String>, // Public crypto address
    pub address_index: Option<i32>, // For crypto subaddresses
    pub status: String, // Ideally map to WalletStatus enum
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Implement FromSql/ToSql for WalletType and WalletStatus if using Diesel enum mapping features

/// Represents data needed to create a new wallet.
#[derive(Debug, Deserialize, Insertable, Clone)]
#[diesel(table_name = wallets)]
pub struct NewWallet<'a> {
    pub user_id: Uuid,
    pub wallet_type: &'a str, // Store as string, map from enum on insert
    pub currency_code: &'a str,
    #[diesel(serialize_as = BigDecimal)] // Map app Decimal to DB Numeric
    pub balance: Option<Decimal>, // Use Option for default DB value
    pub bank_name: Option<&'a str>,
    pub account_number_hash: Option<&'a str>,
    pub iban_hash: Option<&'a str>,
    pub bic_swift: Option<&'a str>,
    pub routing_number_hash: Option<&'a str>,
    pub address: Option<&'a str>,
    pub address_index: Option<i32>,
    pub status: Option<&'a str>, // Store as string, map from enum on insert
     // wallet_id, created_at, updated_at are defaulted by DB/triggers
}

/// Represents data allowed for updating a wallet.
#[derive(Debug, Deserialize, AsChangeset, Clone)]
#[diesel(table_name = wallets)]
pub struct UpdateWallet<'a> {
    #[diesel(serialize_as = Option<BigDecimal>)] // Map update value correctly
    pub balance: Option<Decimal>, // Only update balance if provided
    pub bank_name: Option<&'a str>,
    pub account_number_hash: Option<&'a str>,
    pub iban_hash: Option<&'a str>,
    pub bic_swift: Option<&'a str>,
    pub routing_number_hash: Option<&'a str>,
    pub address: Option<&'a str>,
    pub address_index: Option<i32>,
    pub status: Option<&'a str>, // Update status if provided
     // updated_at is handled by trigger
     // user_id, wallet_type, currency_code usually not updatable
}