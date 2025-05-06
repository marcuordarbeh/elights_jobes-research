// /home/inno/elights_jobes-research/cryptography-exchange/src/error.rs
use thiserror::Error;

/// Errors related to cryptocurrency exchange operations and integrations.
#[derive(Error, Debug)]
pub enum ExchangeError {
    #[error("Missing configuration or environment variable: {0}")]
    ConfigurationError(String),

    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Client is not initialized or configured properly for {0}")]
    NotInitialized(&'static str),

    #[error("API returned an error: Status={status}, Body={body}")]
    ApiError {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("JSON-RPC error from Monero Wallet: Code={code}, Message={message}")]
    JsonRpcError { code: i64, message: String },

    #[error("Failed to parse API/RPC response: {0}")]
    ResponseParseError(String),

    #[error("Operation timed out interacting with service")]
    TimeoutError,

    #[error("Invalid input provided: {0}")]
    InvalidInput(String),

    #[error("Cryptocurrency operation failed: {0}")]
    CryptoOperationFailed(String), // e.g., signing error, address validation

    #[error("Conversion error: {0}")]
    ConversionError(String),

    #[error("Unsupported currency or pair: {0}")]
    UnsupportedCurrency(String),

    #[error("Webhook verification failed: {0}")]
    WebhookVerificationError(String),

    #[error("Rate fetching error: {0}")]
    RateFetchingError(String),

    #[error("Internal client error: {0}")]
    InternalError(String),
}