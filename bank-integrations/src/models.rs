// /home/inno/elights_jobes-research/bank-integrations/src/models.rs
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;

// --- Common Account Information ---
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountInfo {
    pub account_id: String, // Bank's identifier for the account
    pub account_iban: Option<String>,
    pub account_number: Option<String>, // Masked or partial usually
    pub currency: String, // ISO 4217
    pub account_type: String, // e.g., "Checking", "Savings", "Loan"
    pub owner_name: Option<String>,
    pub bank_name: String, // Name of the bank holding the account
    pub status: String, // e.g., "Active", "Closed"
    pub metadata: Option<serde_json::Value>, // Bank-specific extra info
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Balance {
    pub account_id: String,
    pub currency: String,
    pub available_balance: Decimal,
    pub ledger_balance: Decimal, // Also known as book balance
    pub balance_timestamp: DateTime<Utc>,
    pub credit_limit: Option<Decimal>, // For credit accounts
}

// --- Common Transaction Representation ---
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionType {
    Debit,
    Credit,
    Fee,
    Interest,
    TransferIn,
    TransferOut,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BankTransaction {
    pub transaction_id: String, // Bank's unique transaction identifier
    pub account_id: String,
    pub timestamp: DateTime<Utc>, // Booking date/time
    pub value_date: Option<NaiveDate>, // Value date
    pub amount: Decimal,
    pub currency: String,
    pub transaction_type: TransactionType, // Debit or Credit relative to the account
    pub description: String, // Description provided by bank or counterparty
    pub counterparty_name: Option<String>,
    pub counterparty_account: Option<String>, // Masked/partial
    pub counterparty_bank_bic: Option<String>,
    pub running_balance: Option<Decimal>, // Balance after this transaction
    pub metadata: Option<serde_json::Value>, // Bank-specific codes, references etc.
}

// --- Common Payment Initiation ---
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentRequest {
    pub client_reference: String, // Your unique reference for the request
    pub debit_account_id: String, // Account to debit at the bank
    pub credit_account_number: String, // Beneficiary account (IBAN or other)
    pub credit_account_name: String, // Beneficiary name
    pub credit_bank_bic: String, // Beneficiary bank BIC/SWIFT
    pub credit_bank_name: Option<String>,
    pub amount: Decimal,
    pub currency: String,
    pub value_date: Option<NaiveDate>, // Requested execution date
    pub end_to_end_id: Option<String>, // Optional E2E ID for tracking
    pub remittance_info: Option<String>, // Payment reason/reference
    pub charge_bearer: Option<String>, // e.g., "SHA", "BEN", "OUR"
    pub payment_type: String, // e.g., "WIRE", "SEPA_CREDIT", "ACH_CREDIT"
    // Add fields for purpose codes, regulatory reporting etc. if needed
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum PaymentStatusCode {
    Received,      // Bank received the instruction
    Pending,       // Awaiting processing/authorization/liquidity
    Processing,    // Actively being processed by bank/network
    Accepted,      // Accepted for settlement (e.g., ACSP in ISO 20022 pacs.002)
    Rejected,      // Rejected by bank or network
    Settled,       // Final settlement confirmed (funds transferred)
    Cancelled,     // Payment cancelled before settlement
    Unknown,       // Status cannot be determined
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentStatus {
    pub payment_id: String, // Bank's reference for the initiated payment
    pub client_reference: Option<String>, // Your original reference
    pub status_code: PaymentStatusCode,
    pub status_description: Option<String>, // Bank's description or reason code
    pub timestamp: DateTime<Utc>, // Timestamp of this status update
    pub settled_amount: Option<Decimal>, // Amount actually settled (might differ due to FX/fees)
    pub settlement_date: Option<NaiveDate>,
    pub uetr: Option<String>, // SWIFT UETR if available
    pub metadata: Option<serde_json::Value>, // Additional status details
}