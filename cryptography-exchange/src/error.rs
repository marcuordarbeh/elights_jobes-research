// /home/inno/elights_jobes-research/cryptography-exchange/src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExchangeError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("API returned error status {0}: {1}")]
    ApiError(reqwest::StatusCode, String),

    #[error("Failed to parse API response: {0}")]
    ParseError(String),

    #[error("Missing configuration: {0}")]
    MissingConfig(&'static str),

    #[error("Crypto operation failed: {0}")]
    CryptoError(String),

    #[error("Conversion error: {0}")]
    ConversionError(String),

    #[error("Unsupported currency pair: {0} -> {1}")]
    UnsupportedPair(String, String),

    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
}