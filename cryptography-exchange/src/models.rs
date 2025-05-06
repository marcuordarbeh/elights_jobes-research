// /home/inno/elights_jobes-research/cryptography-exchange/src/models.rs
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use uuid::Uuid;

// --- BTCPay Server Specific Models (Derived from swagger.json) ---

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceMetadata {
    #[serde(default)] // Allow optional field
    pub order_id: Option<String>,
    #[serde(default)]
    pub pos_data: Option<String>,
     // Add other custom metadata fields you might use
     #[serde(default)]
    pub internal_tx_id: Option<Uuid>, // Link back to internal transaction
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCheckoutOptions {
    #[serde(default)]
    pub speed_policy: Option<String>, // e.g., "HighSpeed", "MediumSpeed", "LowSpeed"
    #[serde(default)]
    pub payment_methods: Option<Vec<String>>, // e.g., ["BTC", "BTC-LightningNetwork"]
    #[serde(default)]
    pub redirect_url: Option<String>,
    #[serde(default)]
    pub redirect_automatically: Option<bool>,
    // Add other checkout options
}

/// Request to create a BTCPay Invoice.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvoiceRequest {
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
    pub currency: String, // Fiat currency code (e.g., "USD", "EUR")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<InvoiceMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout: Option<InvoiceCheckoutOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<InvoiceReceiptOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalSearchTerms")]
    pub additional_search_terms: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceReceiptOptions {
    pub enabled: Option<bool>,
    // Add other receipt options if needed
}


/// Response representing a BTCPay Invoice (simplified from swagger).
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceData {
    pub id: String,
    pub store_id: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
    pub currency: String,
    pub metadata: Option<InvoiceMetadata>, // Ensure metadata is included if needed
    pub checkout: InvoiceCheckoutOptions, // Included for checkout link usually
    pub status: String, // e.g., "New", "Processing", "Expired", "Invalid", "Settled"
    pub additional_status_info: Option<String>,
    pub expiration_time: Option<i64>, // Unix timestamp (seconds)
    pub monitoring_expiration: Option<i64>,
    pub created_time: i64,
    // TODO: Add payment methods, addresses etc. if needed from detailed response
    // pub paymentMethods: Vec<...>,
    #[serde(rename = "checkoutLink")]
    pub checkout_link: String,
}

/// Represents state of a Payout (from swagger).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum PayoutState {
    AwaitingPayment,
    AwaitingApproval,
    InProgress,
    Completed,
    Cancelled,
}

/// Request to create a BTCPay Payout.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatePayoutRequest {
    pub destination: String, // Crypto address
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal, // Amount in the crypto currency
    pub payment_method: String, // e.g., "BTC", "XMR" - Must match a configured payout processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_payment_id: Option<String>, // If linked to a pull payment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>, // Whether to approve immediately
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// Response representing a BTCPay Payout (from swagger).
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PayoutData {
    pub payout_id: String,
    pub revision: Option<i32>,
    pub pull_payment_id: Option<String>,
    pub date: DateTime<Utc>,
    pub destination: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
    pub payment_method: String,
    pub crypto_code: String, // e.g. "BTC"
    pub payment_method_amount: Option<String>, // Amount in payment method currency (might differ due to rate)
    pub state: PayoutState,
    pub memo: Option<String>,
    // TODO: Add payment_proof if needed
}

/// Webhook data model for Invoice events (simplified).
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebhookInvoiceEvent {
    pub delivery_id: Option<String>,
    pub webhook_id: Option<String>,
    pub original_delivery_id: Option<String>,
    pub is_redelivery: Option<bool>,
    pub r#type: String, // e.g., "InvoiceReceivedPayment", "InvoiceProcessing", "InvoiceSettled", "InvoiceInvalid", "InvoiceExpired"
    pub timestamp: Option<i64>,
    pub store_id: String,
    pub invoice_id: String,
    // Include specific event payload fields based on type if needed
    // e.g., payment: Option<...>, manuallyMarked: Option<bool>
}


// --- Monero Specific Models ---

#[derive(Debug, Deserialize, Clone)]
pub struct MoneroBalance {
    #[serde(with = "crate::utils::serde_monero_atomic")]
    pub balance: u64, // Total balance in atomic units
    #[serde(with = "crate::utils::serde_monero_atomic")]
    pub unlocked_balance: u64, // Spendable balance
    // blocksToUnlock ?
}

#[derive(Debug, Deserialize, Clone)]
pub struct MoneroAddress {
    pub address: String, // Base address or subaddress
    #[serde(rename="addressIndex")]
    pub address_index: Option<u32>, // Subaddress index if applicable
}

#[derive(Debug, Deserialize, Clone)]
pub struct MoneroTransferResult {
    pub amount: Option<u64>, // Amount transferred (atomic units)
    pub fee: u64, // Fee paid (atomic units)
    pub tx_hash: String, // Transaction hash
    pub tx_key: Option<String>, // Transaction secret key (needed to prove payment)
    pub tx_blob: Option<String>, // Raw transaction hex
    // Add other fields like tx_metadata, multisig_txset etc. if needed
}

// --- Common Conversion/Rate Models ---
#[derive(Debug, Deserialize, Clone)]
pub struct ConversionQuote {
    pub from_currency: String,
    pub to_currency: String,
    pub rate: Decimal,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ConversionRequest {
    pub from_currency: String,
    pub to_currency: String,
    pub amount: Decimal,
    // Add flags like "execute" vs "quote_only" if needed
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConversionResult {
    pub from_currency: String,
    pub to_currency: String,
    pub original_amount: Decimal,
    pub converted_amount: Decimal,
    pub rate_used: Decimal,
    pub exchange_reference_id: Option<String>, // ID if executed via external exchange
}