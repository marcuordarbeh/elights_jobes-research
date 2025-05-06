// /home/inno/elights_jobes-research/backend/core-api/src/error.rs
use actix_web::{ResponseError, HttpResponse, http::{StatusCode, header::ContentType}};
use thiserror::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use serde_json::json;

// Define the unified API error type
#[derive(Error)]
pub enum ApiError {
    #[error("Internal Server Error")] // Hide internal details from client
    InternalError(#[from] InternalErrorDetail), // Box the source error

    #[error("Database Error: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("Database Pool Error: {0}")]
    DbPoolError(#[from] r2d2::Error),

    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Authentication Error: {0}")]
    AuthenticationError(String),

    #[error("Authorization Error: {0}")]
    AuthorizationError(String),

    #[error("Configuration Error: {0}")]
    ConfigurationError(String),

    #[error("External Service Error: {0}")]
    ExternalServiceError(String),

    #[error("Timeout Error: {0}")]
    TimeoutError(String),

    // Wrap domain errors
    #[error("Domain Logic Error: {0}")]
    DomainLogicError(#[from] domain::DomainError),

    // Wrap other specific errors as needed
    #[error("HTTP Client Error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

// Implement Debug manually to potentially hide sensitive source details
impl Debug for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Log the detailed error internally
        log::error!("API Error Detail: {:?}", self);
        // Display a user-friendly error message
        write!(f, "{}", self)
    }
}

// Implement ResponseError for Actix integration
impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DbPoolError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            ApiError::AuthorizationError(_) => StatusCode::FORBIDDEN,
            ApiError::ConfigurationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ExternalServiceError(_) => StatusCode::BAD_GATEWAY,
            ApiError::TimeoutError(_) => StatusCode::GATEWAY_TIMEOUT,
            ApiError::ReqwestError(_) => StatusCode::BAD_GATEWAY, // Or INTERNAL_SERVER_ERROR
            ApiError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DomainLogicError(ref domain_err) => match domain_err {
                domain::DomainError::Validation(_) => StatusCode::BAD_REQUEST,
                domain::DomainError::NotFound(_) => StatusCode::NOT_FOUND,
                domain::DomainError::Authentication(_) => StatusCode::UNAUTHORIZED,
                domain::DomainError::Authorization(_) => StatusCode::FORBIDDEN,
                domain::DomainError::InsufficientFunds(_) => StatusCode::BAD_REQUEST, // Or CONFLICT?
                domain::DomainError::NotSupported(_) => StatusCode::NOT_IMPLEMENTED,
                _ => StatusCode::INTERNAL_SERVER_ERROR, // Default internal for other domain errors
            },
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let error_message = match self {
             // Provide specific user-friendly messages for known error types
             ApiError::ValidationError(ref msg) => msg.clone(),
             ApiError::BadRequest(ref msg) => msg.clone(),
             ApiError::NotFound(ref msg) => msg.clone(),
             ApiError::AuthenticationError(ref msg) => msg.clone(),
             ApiError::AuthorizationError(ref msg) => msg.clone(),
             ApiError::DomainLogicError(ref domain_err) => match domain_err {
                 // Expose specific domain errors safely
                 domain::DomainError::Validation(m) => m.clone(),
                 domain::DomainError::NotFound(m) => m.clone(),
                 domain::DomainError::Authentication(m) => m.clone(),
                 domain::DomainError::Authorization(m) => m.clone(),
                  domain::DomainError::InsufficientFunds(_) => "Insufficient funds".to_string(),
                 // Hide internal details for other domain errors
                 _ => "An internal processing error occurred".to_string(),
             },
             // Default generic messages for internal/unexpected errors
             _ => "An unexpected internal error occurred".to_string(),
        };

        HttpResponse::build(status)
            .insert_header(ContentType::json())
            .json(json!({ "error": error_message }))
    }
}


// --- Helper for Internal Errors ---
// Use this to wrap underlying errors while providing a user-friendly message

#[derive(Error)]
pub struct InternalErrorDetail {
    #[source]
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
}

impl Debug for InternalErrorDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self.source) // Log the actual source error
    }
}
impl Display for InternalErrorDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Internal Server Error") // User-facing message
    }
}

// Helper macro or function to create InternalErrorDetail easily
pub fn internal_error<E>(source: E) -> ApiError
where
    E: std::error::Error + Send + Sync + 'static,
{
    ApiError::InternalError(InternalErrorDetail { source: Box::new(source) })
}

// Example conversion from a specific external error type
// impl From<SomeExternalError> for ApiError {
//     fn from(err: SomeExternalError) -> Self {
//         internal_error(err) // Wrap it as an internal error
//     }
// }