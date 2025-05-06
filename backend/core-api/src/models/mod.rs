// /home/inno/elights_jobes-research/backend/core-api/src/models/mod.rs
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use uuid::Uuid;
use domain::models::{TransactionStatus, TransactionType}; // Use domain enums

// --- Auth Models ---
#[derive(Debug, Deserialize)]
pub struct ApiLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiRegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct ApiAuthResponse {
    pub token: String, // JWT Token
    pub expires_at: i64, // Unix timestamp
}

#[derive(Debug, Serialize)]
pub struct ApiRegisterResponse {
    pub message: String,
    pub user_id: Uuid, // Return the new user's ID
}

// --- Payment Models ---
#[derive(Debug, Deserialize)]
pub struct ApiInitiatePaymentRequest {
    pub amount: Decimal,
    pub currency: String,
    pub payment_type: TransactionType,
    pub source_wallet_id: Option<Uuid>, // Internal source
    pub destination_wallet_id: Option<Uuid>, // Internal destination
    // Add nested structs for external details if preferred over flattened structure
    // pub external_details: Option<ExternalPaymentDetails>,
    // Or keep flattened:
    pub beneficiary_name: Option<String>,
    pub beneficiary_account: Option<String>, // IBAN or Account#
    pub beneficiary_bic: Option<String>, // SWIFT BIC
    pub card_token: Option<String>, // Token from frontend/gateway, NOT raw card details
    pub ach_routing: Option<String>,
    pub ach_account: Option<String>,
    // ... other needed fields ...
    pub description: Option<String>,
    pub idempotency_key: Option<String>, // For preventing duplicate requests
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ApiPaymentResponse {
    pub transaction_id: Uuid,
    pub status: TransactionStatus,
    pub message: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    // Optionally include links for status check etc.
}

#[derive(Debug, Serialize)]
pub struct ApiPaymentStatusResponse {
     pub transaction_id: Uuid,
     pub status: TransactionStatus,
     pub transaction_type: TransactionType,
     pub amount: String, // Return decimal as string
     pub currency: String,
     pub description: Option<String>,
     pub created_at: chrono::DateTime<chrono::Utc>,
     pub updated_at: chrono::DateTime<chrono::Utc>,
     pub settlement_at: Option<chrono::DateTime<chrono::Utc>>,
     pub external_ref_id: Option<String>,
     pub metadata: Option<serde_json::Value>,
}


// --- Crypto Models ---
#[derive(Debug, Deserialize)]
pub struct ApiCryptoConversionRequest {
    pub amount: Decimal,
    pub from_currency: String, // e.g., "BTC", "XMR", "USD", "EUR"
    pub to_currency: String,
}

#[derive(Debug, Serialize)]
pub struct ApiCryptoConversionResponse {
    pub original_amount: String,
    pub converted_amount: String,
    pub from_currency: String,
    pub to_currency: String,
    pub rate: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ApiCryptoWithdrawalRequest {
    pub source_wallet_id: Uuid, // Internal wallet ID
    pub amount: Decimal,
    // Currency inferred from wallet usually
    pub destination_address: String,
    pub payment_id: Option<String>, // For Monero
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiCryptoWithdrawalResponse {
     pub transaction_id: Uuid, // Internal transaction ID tracking the withdrawal
     pub status: TransactionStatus,
     pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ApiWalletBalanceResponse {
    pub wallet_id: Uuid,
    pub currency: String,
    pub balance: String, // Decimal as string
    pub wallet_type: String,
    pub address: Option<String>, // Public address if applicable
}

// --- FT Models ---
#[derive(Debug, Deserialize)]
pub struct FtNotificationPayload {
    // Define structure based on FT Push Notification docs
    // Example:
    pub notifications: Vec<FtNotificationItem>,
}

#[derive(Debug, Deserialize)]
pub struct FtNotificationItem {
    #[serde(rename = "type")]
    pub notification_type: String, // e.g., "http://www.ft.com/thing/ThingChangeType/UPDATE"
    #[serde(rename = "apiUrl")]
    pub api_url: String, // URL to fetch the updated content
    #[serde(rename = "lastModified")]
    pub last_modified: Option<DateTime<chrono::Utc>>,
    #[serde(rename = "publishReference")]
    pub publish_reference: Option<String>,
}

// --- Generic API Responses ---
#[derive(Debug, Serialize)]
pub struct ApiMessageResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
}