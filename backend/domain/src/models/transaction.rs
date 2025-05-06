// /home/inno/elights_jobes-research/backend/domain/src/models/transaction.rs
use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::{table, sql_types::{Uuid as DieselUuid, Nullable, Varchar, Numeric as DieselNumeric, Text, Jsonb, Timestamptz}};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;
use serde_json::Value as JsonValue; // For metadata JSONB
use bigdecimal::BigDecimal;

// Import schema definition (resolve path as needed)
// use schema::core_schema::transactions;
// TODO: Resolve schema path access. Using direct table reference for now.
table! {
     core_schema.transactions (transaction_id) {
        transaction_id -> DieselUuid,
        debit_wallet_id -> Nullable<DieselUuid>,
        credit_wallet_id -> Nullable<DieselUuid>,
        transaction_type -> Varchar,
        status -> Varchar,
        amount -> DieselNumeric,
        currency_code -> Varchar,
        description -> Nullable<Text>,
        external_ref_id -> Nullable<Varchar>,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        settlement_at -> Nullable<Timestamptz>,
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "core_schema.sql_types::TransactionType"] // Assuming TYPE transaction_type
pub enum TransactionType {
    AchCredit,
    AchDebit,
    WireOutbound,
    WireInbound,
    CardAuthorization,
    CardCapture,
    CardRefund,
    CardChargeback, // Added
    CheckDeposit,
    CheckWithdrawal, // Added
    CryptoBtcSend,
    CryptoBtcReceive,
    CryptoXmrSend,
    CryptoXmrReceive,
    InternalTransfer,
    Conversion, // Added for currency conversions
    Fee, // Added for representing fees
    RtgsCreditTransfer, // Specific for TARGET2/RTGS ISO 20022 pacs.008
    RtgsDirectDebit, // Specific for TARGET2/RTGS ISO 20022 pacs.003 (less common for RTGS)
    RtgsReturn, // Specific for TARGET2/RTGS ISO 20022 pacs.004
    RtgsStatusUpdate, // Representing camt messages? Or internal status.
    Unknown, // Default/Fallback
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "core_schema.sql_types::TransactionStatus"] // Assuming TYPE transaction_status
pub enum TransactionStatus {
    Pending,          // Initial state
    Processing,       // Actively being processed (e.g., sent to bank)
    RequiresAction,   // Needs user input (e.g., 2FA, clarification)
    Authorized,       // Card payment authorized, awaiting capture
    Submitted,        // Submitted to network (ACH, Wire, Crypto broadcast)
    Settled,          // Final settlement confirmed (RTGS, some ACH)
    Completed,        // Successfully processed and funds delivered/credited
    Failed,           // Processing failed permanently
    Cancelled,        // Cancelled by user or system before completion
    Returned,         // Payment returned (ACH, Wire)
    Chargeback,       // Card chargeback initiated
    Expired,          // Timed out (e.g., crypto invoice)
}

/// Represents a financial transaction in the system.
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable, Clone, PartialEq)]
#[diesel(table_name = transactions, primary_key(transaction_id))]
pub struct Transaction {
    pub transaction_id: Uuid,
    pub debit_wallet_id: Option<Uuid>,
    pub credit_wallet_id: Option<Uuid>,
    pub transaction_type: String, // Map to TransactionType enum
    pub status: String, // Map to TransactionStatus enum
    #[diesel(deserialize_as = BigDecimal)]
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
    pub currency_code: String,
    pub description: Option<String>,
    pub external_ref_id: Option<String>, // e.g., Bank ref, Crypto Tx Hash
    pub metadata: Option<JsonValue>, // Store type-specific details
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub settlement_at: Option<DateTime<Utc>>, // Track final settlement time
}


/// Represents data needed to create a new transaction.
#[derive(Debug, Deserialize, Insertable, Clone)]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    pub transaction_id: Option<Uuid>, // Allow generating outside DB if needed
    pub debit_wallet_id: Option<Uuid>,
    pub credit_wallet_id: Option<Uuid>,
    pub transaction_type: &'a str, // Store as string, map from enum
    pub status: &'a str, // Store as string, map from enum
    #[diesel(serialize_as = BigDecimal)]
    pub amount: Decimal,
    pub currency_code: &'a str,
    pub description: Option<&'a str>,
    pub external_ref_id: Option<&'a str>,
    pub metadata: Option<JsonValue>,
    // created_at, updated_at defaulted by DB
    // settlement_at is set later
}

/// Represents data allowed for updating a transaction.
#[derive(Debug, Deserialize, AsChangeset, Clone)]
#[diesel(table_name = transactions)]
pub struct UpdateTransaction<'a> {
    pub status: Option<&'a str>, // Update status
    pub external_ref_id: Option<&'a str>, // Update external reference
    pub metadata: Option<JsonValue>, // Merge or replace metadata
    pub settlement_at: Option<DateTime<Utc>>, // Set settlement time
     // updated_at handled by trigger
     // Other fields usually not updatable after creation
}

// --- Structs for Metadata (Examples) ---
// These can be serialized/deserialized into the JSONB metadata field

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardDetails {
    pub authorization_code: Option<String>,
    pub network_reference_id: Option<String>, // Visa/MC ref
    pub last4: Option<String>, // Masked card number
    // Add other relevant non-sensitive card details
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AchDetails {
    pub trace_number: Option<String>, // NACHA trace number
    pub return_code: Option<String>, // If returned
    pub company_id: Option<String>, // SEC code specific IDs
    pub effective_entry_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WireDetails {
    pub uetr: Option<String>, // Unique End-to-end Transaction Reference (SWIFT gpi)
    pub swift_messages: Option<Vec<String>>, // Store raw MT/MX messages if needed (or refs)
    pub intermediary_banks: Option<Vec<BankIdentifier>>, // List of intermediary banks
    pub purpose_code: Option<String>,
    pub remittance_info: Option<String>, // Structured or unstructured remittance
    pub charge_details: Option<String>, // BEN, OUR, SHA
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CheckDetails {
    pub check_number: Option<String>,
    pub deposit_method: Option<String>, // e.g., 'REMOTE_CAPTURE', 'BRANCH'
    pub image_reference: Option<String>, // Reference to check images if stored
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CryptoDetails {
    pub network_tx_hash: Option<String>,
    pub block_height: Option<u64>,
    pub network_fee: Option<String>, // Fee amount as string for precision
    pub destination_tag: Option<String>, // For cryptos like XRP
    pub payment_id: Option<String>, // For Monero
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BankIdentifier {
    pub name: Option<String>,
    pub bic_swift: Option<String>,
    pub clearing_code: Option<String>, // e.g., Fedwire ABA, CHIPS UID
    pub country_code: Option<String>, // ISO Country Code
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)] // Allows metadata to store one of these types directly
pub enum PaymentDetails {
    Card(CardDetails),
    Ach(AchDetails),
    Wire(WireDetails),
    Check(CheckDetails),
    Crypto(CryptoDetails),
    // Add others if needed
}