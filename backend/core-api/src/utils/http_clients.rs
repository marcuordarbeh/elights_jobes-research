// /home/inno/elights_jobes-research/backend/core-api/src/utils/http_clients.rs
use crate::config::AppConfig;
use crate::error::ApiError;
use reqwest::{Client as HttpClient, Proxy};
use std::time::Duration;

/// Holds configured reqwest HTTP clients for use throughout the application.
#[derive(Clone)] // Cloneable for sharing via web::Data
pub struct HttpClients {
    /// Standard client for general external API calls.
    pub standard_client: HttpClient,
    /// Client configured to use the Tor SOCKS proxy (if enabled).
    pub tor_client: Option<HttpClient>, // Optional based on config
}

/// Initializes the shared HTTP clients based on AppConfig.
pub fn init_http_clients(config: &AppConfig) -> Result<HttpClients, ApiError> {
    // --- Standard Client ---
    let standard_client = HttpClient::builder()
        .timeout(Duration::from_secs(30)) // Default timeout
        .user_agent(format!("ElightsCoreAPI/{}", env!("CARGO_PKG_VERSION")))
        // TODO: Add connection pooling options if needed
        .build()
        .map_err(|e| ApiError::ConfigurationError(format!("Failed to build standard HTTP client: {}", e)))?;
    log::info!("Standard HTTP client initialized.");

    // --- Tor Client (Optional) ---
    let tor_client = if let Some(proxy_addr) = &config.tor_socks_proxy {
        log::info!("Attempting to configure Tor SOCKS proxy at: {}", proxy_addr);
        // Ensure the proxy address includes the scheme (socks5h for DNS resolution via Tor)
        let full_proxy_url = if !proxy_addr.starts_with("socks5h://") && !proxy_addr.starts_with("socks5://") {
            format!("socks5h://{}", proxy_addr) // Prefer socks5h
        } else {
            proxy_addr.clone()
        };

        match Proxy::all(&full_proxy_url) {
            Ok(proxy) => {
                log::info!("Tor proxy configured successfully: {}", full_proxy_url);
                 HttpClient::builder()
                    .proxy(proxy)
                    .timeout(Duration::from_secs(120)) // Increase timeout for Tor potentially
                    .user_agent(format!("ElightsCoreAPI-Tor/{}", env!("CARGO_PKG_VERSION")))
                    .build()
                    .map_err(|e| ApiError::ConfigurationError(format!("Failed to build Tor HTTP client: {}", e)))
                    .ok() // Make it Option<HttpClient>
            },
            Err(e) => {
                 log::error!("Failed to configure Tor SOCKS proxy '{}': {}. Tor client disabled.", proxy_addr, e);
                 None // Disable Tor client if proxy setup fails
            }
        }
    } else {
        log::info!("Tor SOCKS proxy not configured. Tor client disabled.");
        None
    };

    if tor_client.is_some() {
         log::info!("Tor-enabled HTTP client initialized.");
    }

    Ok(HttpClients {
        standard_client,
        tor_client,
    })
}