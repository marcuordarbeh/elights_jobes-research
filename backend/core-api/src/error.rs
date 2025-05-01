// /home/inno/elights_jobes-research/backend/core-api/src/error.rs
use actix_web::{ResponseError, HttpResponse};
use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Internal Server Error: {0}")]
    InternalError(String),

    #[error("Database Error: {0}")]
    DatabaseError(String),

    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("Not Found Error: {0}")]
    NotFoundError(String),

    #[error("Authentication Error: {0}")]
    AuthenticationError(String),

    #[error("Authorization Error: {0}")]
    AuthorizationError(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Configuration Error: {0}")]
    ConfigurationError(String),

    // Specific error for wrapping domain errors
    #[error("Domain Logic Error: {0}")]
    DomainLogicError(#[from] domain::DomainError), // Assuming domain crate error is DomainError

     // Specific error for wrapping SQLx errors
    #[error("SQLx Error: {0}")]
    SqlxError(#[from] sqlx::Error),

     // Specific error for wrapping Reqwest errors (if clients used directly here)
     #[error("HTTP Client Error: {0}")]
     ReqwestError(#[from] reqwest::Error),
}

// Implement ResponseError for ApiError to integrate with Actix
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        log::error!("API Error: {}", self); // Log the error details server-side

        match *self {
            ApiError::InternalError(_) => HttpResponse::InternalServerError().json("Internal Server Error"),
            ApiError::DatabaseError(_) => HttpResponse::InternalServerError().json("Database Error"),
            ApiError::NotFoundError(ref message) => HttpResponse::NotFound().json(message),
            ApiError::ValidationError(ref message) => HttpResponse::BadRequest().json(message),
            ApiError::AuthenticationError(ref message) => HttpResponse::Unauthorized().json(message),
            ApiError::AuthorizationError(ref message) => HttpResponse::Forbidden().json(message),
            ApiError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ApiError::ConfigurationError(_) => HttpResponse::InternalServerError().json("Configuration Error"),
            ApiError::DomainLogicError(ref err) => {
                 // Map domain errors to appropriate HTTP responses
                 match err {
                     domain::DomainError::NotFound(m) => HttpResponse::NotFound().json(m),
                     domain::DomainError::Validation(m) => HttpResponse::BadRequest().json(m),
                     domain::DomainError::Security(m) => HttpResponse::Unauthorized().json(m),
                     _ => HttpResponse::InternalServerError().json("An unexpected error occurred"),
                 }
            }
             ApiError::SqlxError(_) => HttpResponse::InternalServerError().json("Database operation failed"),
             ApiError::ReqwestError(_) => HttpResponse::BadGateway().json("Failed to communicate with external service"),
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            ApiError::NotFoundError(_) => actix_web::http::StatusCode::NOT_FOUND,
            ApiError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::AuthenticationError(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            ApiError::AuthorizationError(_) => actix_web::http::StatusCode::FORBIDDEN,
            ApiError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::DomainLogicError(ref err) => match err { // Map domain status codes
                 domain::DomainError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
                 domain::DomainError::Validation(_) => actix_web::http::StatusCode::BAD_REQUEST,
                 domain::DomainError::Security(_) => actix_web::http::StatusCode::UNAUTHORIZED,
                 _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
             },
             ApiError::ReqwestError(_) => actix_web::http::StatusCode::BAD_GATEWAY,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}