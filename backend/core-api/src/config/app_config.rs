// /home/inno/elights_jobes-research/backend/core-api/src/config/app_config.rs
use crate::error::ApiError;
use dotenv::dotenv;
use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use std::collections::HashSet;
use once_cell::sync::Lazy; // Use Lazy for static config

// Define a struct to hold all application configurations loaded from environment
#[derive(Debug, Clone)]
pub struct AppConfig {
    // Server Config
    pub api_bind_address: String,
    pub tls_enabled: bool, // Flag to enable/disable TLS binding in main.rs
    pub tls_bind_address: Option<String>,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,

    // Database Config
    pub database_url: String,

    // Security Config
    pub jwt_secret: String,
    pub jwt_duration_hours: u32,
    pub allowed_ips: HashSet<IpAddr>, // For IP whitelist middleware

    // Integration Configs
    pub btcpay_url: String,
    pub btcpay_api_key: String,
    pub btcpay_default_store_id: String, // Assume one primary store ID
    pub monero_wallet_rpc: String,
    pub monero_wallet_user: Option<String>,
    pub monero_wallet_password: Option<String>,
    pub tor_socks_proxy: Option<String>, // Address for Tor proxy, e.g., "127.0.0.1:9050"

    // FT API Config
    pub ft_api_key: String,
    pub ft_push_enabled: bool,
    pub ft_push_callback_url: Option<String>, // URL core-api listens on

    // Bank API Keys (add all required ones)
    pub chase_api_key: String,
    pub wells_fargo_api_key: String,
    // ... other bank keys/tokens ...
    pub deutsche_bank_client_id: String,
    pub deutsche_bank_client_secret: String,

    // Add other config sections as needed
}

impl AppConfig {
    /// Loads configuration from environment variables. Panics on missing required vars.
    pub fn load() -> Result<Self, ApiError> {
        dotenv().ok(); // Load .env file

        log::info!("Loading application configuration from environment...");

        let allowed_ips_str = env::var("ALLOWED_IPS").unwrap_or_default();
        let allowed_ips = allowed_ips_str
            .split(',')
            .filter_map(|ip_str| IpAddr::from_str(ip_str.trim()).ok())
            .collect::<HashSet<IpAddr>>();
        if !allowed_ips_str.is_empty() { // Log only if set
             log::debug!("Allowed IPs loaded: {:?}", allowed_ips);
        }


        Ok(AppConfig {
            // Server
            api_bind_address: get_env("API_BIND_ADDR")?,
            tls_enabled: get_env_parse::<bool>("TLS_ENABLED").unwrap_or(false), // Default false
            tls_bind_address: env::var("TLS_BIND_ADDR").ok(), // Optional
            tls_cert_path: env::var("TLS_CERT_PATH").ok(), // Optional
            tls_key_path: env::var("TLS_KEY_PATH").ok(), // Optional

            // Database
            database_url: get_env("DATABASE_URL")?,

            // Security
            jwt_secret: get_env("JWT_SECRET")?,
            jwt_duration_hours: get_env_parse("JWT_DURATION_HOURS")?,
            allowed_ips,

            // Integrations
            btcpay_url: get_env("BTCPAY_URL")?,
            btcpay_api_key: get_env("BTCPAY_API_KEY")?,
            btcpay_default_store_id: get_env("BTCPAY_DEFAULT_STORE_ID")?, // Make this required
            monero_wallet_rpc: get_env("MONERO_WALLET_RPC")?,
            monero_wallet_user: env::var("MONERO_WALLET_USER").ok(),
            monero_wallet_password: env::var("MONERO_WALLET_PASSWORD").ok(),
            tor_socks_proxy: env::var("TOR_SOCKS_PROXY").ok(),

            // FT API
            ft_api_key: get_env("FT_API_KEY")?,
            ft_push_enabled: get_env_parse::<bool>("FT_PUSH_ENABLED").unwrap_or(false),
            ft_push_callback_url: env::var("FT_PUSH_CALLBACK_URL").ok(),

            // Bank API Keys
            chase_api_key: get_env("CHASE_API_KEY")?,
            wells_fargo_api_key: get_env("WELLS_FARGO_API_KEY")?,
            // ... load other keys ...
             deutsche_bank_client_id: get_env("DEUTSCHE_BANK_CLIENT_ID")?,
             deutsche_bank_client_secret: get_env("DEUTSCHE_BANK_CLIENT_SECRET")?,
        })
    }
}

// Helper to get required environment variable
fn get_env(var_name: &str) -> Result<String, ApiError> {
    env::var(var_name).map_err(|e| {
        ApiError::ConfigurationError(format!("Missing required environment variable '{}': {}", var_name, e))
    })
}

// Helper to get and parse environment variable
fn get_env_parse<T: FromStr>(var_name: &str) -> Result<T, ApiError>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    let val_str = get_env(var_name)?;
    val_str.parse::<T>().map_err(|e| {
        ApiError::ConfigurationError(format!("Invalid format for environment variable '{}': {}", var_name, e))
    })
}

// Static instance of the loaded config (can be used across threads)
pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    AppConfig::load().expect("Failed to load application configuration")
});