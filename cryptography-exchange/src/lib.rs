// /home/inno/elights_jobes-research/cryptography-exchange/src/lib.rs

pub mod error;        // Error types
pub mod models;       // Common request/response models
pub mod btcpay;       // BTCPay Server client
pub mod monero_wallet; // Monero Wallet RPC client
pub mod conversion;   // Conversion logic
pub mod rate_service; // Rate fetching service (interface/stub)
pub mod utils;        // Utility functions (e.g., webhook verification)

// Re-export key components
pub use error::ExchangeError;
pub use models::*;
pub use btcpay::BTCPayClient;
#[cfg(feature = "monero_support")] // Only export if feature enabled
pub use monero_wallet::MoneroWalletRpcClient;
pub use conversion::{CurrencyConverter, MockCurrencyConverter}; // Export converter trait/mock
pub use rate_service::{RateService, MockRateService}; // Export rate service trait/mock