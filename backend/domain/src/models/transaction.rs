// /home/inno/elights_jobes-research/backend/domain/src/models/transaction.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use serde_json::Value as JsonValue; // For metadata

// use crate::schema::core_schema::transactions; // For Diesel
// use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TransactionType {
    Ach,
    Wire,
    CryptoBtc,
    CryptoXmr,
    InternalTransfer,
    Card,
    // Add other types as needed
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TransactionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
    RequiresAction,
    // Add other statuses as needed
}

// #[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable)] // Add Diesel traits
// #[diesel(table_name = transactions)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Transaction {
    pub id: Uuid, // UUID primary key
    pub debit_account_id: Option<i32>, // Source account (nullable)
    pub credit_account_id: Option<i32>, // Destination account (nullable)
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
    pub currency: String,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    pub description: Option<String>,
    pub metadata: Option<JsonValue>, // Store transaction-specific details (hashes, SWIFT refs, etc.)
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Add NewTransaction struct for Diesel if needed
// #[derive(Insertable)]
// #[diesel(table_name = transactions)]
// pub struct NewTransaction { ... }