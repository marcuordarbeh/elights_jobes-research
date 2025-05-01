// /home/inno/elights_jobes-research/backend/core-api/src/config/bank_server.rs
use serde::Deserialize; // Keep if deserializing from a file later
use crate::error::ApiError;
use std::env;

// Configuration for the TLS server endpoint (if core-api exposes one)
#[derive(Debug, Clone)] // No need for Deserialize if only loading from env
pub struct BankServerConfig {
    pub host: String,
    pub port: u16,
    pub server_cert: String, // Path to cert file
    pub server_key: String,  // Path to key file
}

impl BankServerConfig {
     /// Loads Bank Server config from environment variables.
    pub fn from_env() -> Result<Self, ApiError> {
         dotenv::dotenv().ok();

         let host = env::var("BANK_HOST")
             .map_err(|_| ApiError::ConfigurationError("BANK_HOST environment variable not set".to_string()))?;
         let port_str = env::var("BANK_PORT")
              .map_err(|_| ApiError::ConfigurationError("BANK_PORT environment variable not set".to_string()))?;
         let port = port_str.parse::<u16>()
              .map_err(|e| ApiError::ConfigurationError(format!("Invalid BANK_PORT value '{}': {}", port_str, e)))?;
         let server_cert = env::var("BANK_SERVER_CERT")
              .map_err(|_| ApiError::ConfigurationError("BANK_SERVER_CERT environment variable not set".to_string()))?;
         let server_key = env::var("BANK_SERVER_KEY")
              .map_err(|_| ApiError::ConfigurationError("BANK_SERVER_KEY environment variable not set".to_string()))?;

        Ok(BankServerConfig {
            host,
            port,
            server_cert,
            server_key,
        })
    }
}