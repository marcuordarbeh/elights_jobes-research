// /home/inno/elights_jobes-research/backend/domain/src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid input: {0}")]
    Validation(String),

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Payment processing failed: {0}")]
    PaymentProcessing(String),

    #[error("Cryptography error: {0}")]
    Cryptography(String),

    #[error("Security error: {0}")]
    Security(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("External service error: {0}")]
    ExternalService(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

// Implement conversions from other error types if needed
// impl From<SomeOtherError> for DomainError { ... }