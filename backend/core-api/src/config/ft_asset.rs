// /home/inno/elights_jobes-research/backend/core-api/src/config/ft_asset.rs
use serde::Deserialize;
use crate::error::ApiError;
use std::net::{IpAddr, AddrParseError};

// Configuration for interacting with FT Asset Management (example)
#[derive(Deserialize, Debug, Clone)]
pub struct FtAssetConfig {
    pub url: String,
    pub client_cert_path: Option<String>, // Path to cert file
    pub client_key_path: Option<String>,  // Path to key file
    pub allowed_ips: Vec<IpAddr>,         // Parsed IP addresses
}

impl FtAssetConfig {
    /// Loads FT Asset config from environment variables.
    pub fn from_env() -> Result<Self, ApiError> {
        dotenv::dotenv().ok();

        let url = env::var("FT_ASSET_URL")
            .map_err(|_| ApiError::ConfigurationError("FT_ASSET_URL environment variable not set".to_string()))?;
        let client_cert_path = env::var("FT_ASSET_CLIENT_CERT").ok();
        let client_key_path = env::var("FT_ASSET_CLIENT_KEY").ok();

        let allowed_ips_str = env::var("FT_ASSET_ALLOWED_IPS").unwrap_or_default();
        let allowed_ips: Result<Vec<IpAddr>, AddrParseError> = allowed_ips_str
            .split(',')
            .filter(|s| !s.trim().is_empty()) // Handle empty strings from split
            .map(|ip_str| ip_str.trim().parse())
            .collect();

        let allowed_ips = allowed_ips
            .map_err(|e| ApiError::ConfigurationError(format!("Invalid IP address in FT_ASSET_ALLOWED_IPS: {}", e)))?;

        Ok(FtAssetConfig {
            url,
            client_cert_path,
            client_key_path,
            allowed_ips,
        })
    }
}