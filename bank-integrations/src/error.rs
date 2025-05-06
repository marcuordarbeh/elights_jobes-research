// /home/inno/elights_jobes-research/bank-integrations/src/error.rs
use thiserror::Error;

/// Common errors encountered when interacting with bank APIs.
#[derive(Error, Debug)]
pub enum BankClientError {
    #[error("Missing configuration or environment variable: {0}")]
    ConfigurationError(String),

    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Authentication failed with bank API: {0}")]
    AuthenticationError(String), // Include reason if possible

    #[error("Bank API returned an error: Status={status}, Body={body}")]
    ApiError {
        status: reqwest::StatusCode,
        body: String, // Include error body from bank if available
    },

    #[error("Failed to parse bank API response: {0}")]
    ResponseParseError(String),

    #[error("Operation timed out connecting to bank API")]
    TimeoutError,

    #[error("Invalid input provided for bank operation: {0}")]
    InvalidInput(String),

    #[error("Bank operation failed: {0}")]
    OperationFailed(String), // General failure reported by bank

    #[error("Feature not supported by this bank integration: {0}")]
    NotSupported(String),

    #[error("Internal client error: {0}")]
    InternalError(String),
}