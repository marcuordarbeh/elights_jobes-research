// /home/inno/elights_jobes-research/backend/core-api/src/config/tls_config.rs
use crate::error::ApiError;
use rustls::ServerConfig;
use std::sync::Arc;

/// Loads the TLS server configuration using the domain helper function.
pub fn load_server_tls_config(
    cert_path: &Option<String>,
    key_path: &Option<String>,
) -> Result<Arc<ServerConfig>, ApiError> {
    match (cert_path, key_path) {
        (Some(cert), Some(key)) => {
            domain::security::tls::load_tls_config(cert, key)
                 .map_err(ApiError::DomainLogicError) // Wrap domain error
        },
        _ => Err(ApiError::ConfigurationError("TLS certificate or key path not provided".to_string())),
    }
}