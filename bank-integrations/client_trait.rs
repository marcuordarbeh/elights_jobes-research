// /home/inno/elights_jobes-research/bank-integrations/src/client_trait.rs
use crate::error::BankClientError;
use crate::models::{AccountInfo, BankTransaction, PaymentRequest, PaymentStatus, Balance};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// Defines the common interface for interacting with different bank APIs.
#[async_trait]
pub trait BankClient: Send + Sync { // Ensure Send + Sync for use across threads

    /// Gets the name of the bank this client connects to.
    fn bank_name(&self) -> &'static str;

    /// Fetches information about a specific bank account.
    async fn fetch_account_info(&self, account_id: &str) -> Result<AccountInfo, BankClientError>;

    /// Fetches the balance(s) for a specific bank account.
    async fn fetch_balance(&self, account_id: &str) -> Result<Balance, BankClientError>;

    /// Lists transactions for a specific bank account within a date range.
    async fn list_transactions(
        &self,
        account_id: &str,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        limit: Option<u32>, // Optional limit for pagination
    ) -> Result<Vec<BankTransaction>, BankClientError>;

    /// Initiates a payment (e.g., Wire, SEPA Credit Transfer) via the bank API.
    async fn initiate_payment(&self, payment_request: &PaymentRequest) -> Result<PaymentStatus, BankClientError>;

    /// Retrieves the status of a previously initiated payment.
    async fn get_payment_status(&self, payment_id: &str) -> Result<PaymentStatus, BankClientError>;

    // TODO: Add other common banking operations as needed:
    // - Initiate Direct Debit
    // - Get Standing Orders
    // - Get Beneficiary List
    // - Foreign Exchange Quote/Execution
}