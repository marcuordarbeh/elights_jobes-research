// /home/inno/elights_jobes-research/bank-integrations/src/lib.rs

pub mod usa;
pub mod europe;
pub mod simulators; // For test_bank_simulators.rs

// Optional: Define a common trait for bank clients
// use async_trait::async_trait;
// use crate::usa::AccountInfo; // Example struct
// use crate::europe::Transaction; // Example struct
// use thiserror::Error;

// #[derive(Debug, Error)]
// pub enum BankClientError {
//     #[error("Authentication failed")]
//     Authentication,
//     #[error("API request failed: {0}")]
//     Request(String),
//     #[error("Parsing response failed: {0}")]
//     Parsing(String),
//     #[error("Feature not supported by bank")]
//     NotSupported,
//     #[error("Internal error: {0}")]
//     Internal(String),
// }

// #[async_trait]
// pub trait BankClient {
//     async fn fetch_account_info(&self, account_id: &str) -> Result<AccountInfo, BankClientError>;
//     async fn list_transactions(&self, account_id: &str, date_range: Option<(String, String)>) -> Result<Vec<Transaction>, BankClientError>;
//     // Add other common methods like initiate_payment, get_payment_status etc.
// }