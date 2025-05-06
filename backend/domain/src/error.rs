// /home/inno/elights_jobes-research/backend/domain/src/error.rs
use thiserror::Error;

/// Represents errors originating from the domain logic.
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Database error: {0}")]
    Database(String), // Generic database error

    #[error("Diesel query error: {0}")]
    DieselError(#[from] diesel::result::Error), // Specific Diesel errors

    #[error("Invalid input: {0}")]
    Validation(String),

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Payment processing failed: {0}")]
    PaymentProcessing(String),

    #[error("Card processing failed: {0}")]
    CardProcessing(String), // Specific card error

    #[error("ACH processing failed: {0}")]
    AchProcessing(String), // Specific ACH error

    #[error("Wire transfer failed: {0}")]
    WireTransfer(String), // Specific Wire error

    #[error("Check processing failed: {0}")]
    CheckProcessing(String), // Specific Check error

    #[error("RTGS settlement error: {0}")]
    RtgsSettlement(String), // Errors related to RTGS interaction

    #[error("Cryptography error: {0}")]
    Cryptography(String),

    #[error("Security error: {0}")]
    Security(String),

    #[error("Authentication failed: {0}")]
    Authentication(String), // Specific auth error

    #[error("Authorization failed: {0}")]
    Authorization(String), // Specific authorization error

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("External service error: {0}")]
    ExternalService(String),

    #[error("Insufficient funds: Wallet ID {0}")]
    InsufficientFunds(uuid::Uuid),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Operation not supported: {0}")]
    NotSupported(String),

    #[error("Network communication error: {0}")]
    NetworkError(String),
}

// Optional: Add conversions from other specific errors if needed
// impl From<bcrypt::BcryptError> for DomainError {
//     fn from(err: bcrypt::BcryptError) -> Self {
//         DomainError::Security(format!("Password hashing/verification error: {}", err))
//     }
// }
//
// impl From<reqwest::Error> for DomainError {
//     fn from(err: reqwest::Error) -> Self {
//         DomainError::NetworkError(format!("HTTP request failed: {}", err))
//     }
// }