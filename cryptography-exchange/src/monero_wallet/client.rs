// /home/inno/elights_jobes-research/cryptography-exchange/src/monero_wallet/client.rs
use crate::error::ExchangeError;
use crate::models::{MoneroBalance, MoneroAddress, MoneroTransferResult};
use crate::monero_wallet::json_rpc::{JsonRpcRequest, JsonRpcResponse, GetBalanceParams, GetAddressParams, TransferParams, Destination};
use reqwest::{Client as HttpClient, Method, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;
use rust_decimal::Decimal;

#[cfg(feature = "monero_support")]
use monero::Address; // Use monero crate for address validation if feature enabled
#[cfg(feature = "monero_support")]
use std::str::FromStr;

#[derive(Clone)]
pub struct MoneroWalletRpcClient {
    http_client: HttpClient,
    rpc_url: String,
    rpc_user: Option<String>,
    rpc_password: Option<String>,
}

impl MoneroWalletRpcClient {
    /// Creates a new Monero Wallet RPC Client from environment variables.
    pub fn new() -> Result<Self, ExchangeError> {
        dotenv::dotenv().ok();

        let rpc_url = env::var("MONERO_WALLET_RPC")
            .map_err(|_| ExchangeError::ConfigurationError("MONERO_WALLET_RPC URL not set".to_string()))?;
        // Credentials are optional depending on wallet RPC setup
        let rpc_user = env::var("MONERO_WALLET_USER").ok();
        let rpc_password = env::var("MONERO_WALLET_PASSWORD").ok();

        let http_client = HttpClient::builder()
            .timeout(std::time::Duration::from_secs(60)) // Wallet RPC calls can take longer
            // TODO: Configure proxy if needed (Tor)
            // .proxy(reqwest::Proxy::all(&env::var("TOR_SOCKS_PROXY")?)?)
            .build()
            .map_err(|e| ExchangeError::InternalError(format!("Failed to build HTTP client: {}", e)))?;

        log::info!("MoneroWalletRpcClient initialized for URL: {}", rpc_url);
        Ok(Self {
            http_client,
            rpc_url,
            rpc_user,
            rpc_password,
        })
    }

    /// Helper to make JSON-RPC calls.
    async fn call_rpc<T: Serialize, R: DeserializeOwned>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R, ExchangeError> {
        let rpc_request = JsonRpcRequest::new(method, params);
        let request_body = serde_json::to_string(&rpc_request)
            .map_err(|e| ExchangeError::InternalError(format!("Failed to serialize RPC request: {}", e)))?;

        let mut request_builder = self.http_client
            .post(&format!("{}/json_rpc", self.rpc_url)) // Common endpoint path
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(request_body);

        // Add digest authentication if username/password are provided
        if let (Some(user), Some(pass)) = (&self.rpc_user, &self.rpc_password) {
            // Reqwest doesn't have built-in digest auth, needs manual implementation or external crate.
            // Basic Auth might work for some setups, but Digest is standard for monero-wallet-rpc.
            // Placeholder using Basic Auth for now, **replace with Digest if needed**.
             log::warn!("Using Basic Auth for Monero RPC - Digest Auth is recommended but requires manual implementation or specific crate.");
             request_builder = request_builder.basic_auth(user, Some(pass));
            // TODO: Implement Digest authentication if required by the RPC server.
        }

        let response = request_builder.send().await?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            log::error!("Monero RPC HTTP Error: Status={}, Body={}", status, body);
            // Treat non-200 HTTP status as API error for RPC
            return Err(ExchangeError::ApiError { status, body });
        }

        // Parse the JSON-RPC response wrapper
        let rpc_response = response.json::<JsonRpcResponse<R>>().await.map_err(|e| {
            log::error!("Failed to parse Monero RPC response JSON: {}", e);
            ExchangeError::ResponseParseError(e.to_string())
        })?;

        // Extract result or handle JSON-RPC level error
        rpc_response.into_result()
    }

    // --- RPC Methods ---

    /// Gets the wallet balance (total and unlocked).
    /// RPC Method: `get_balance`
    pub async fn get_balance(&self) -> Result<MoneroBalance, ExchangeError> {
        log::debug!("Monero RPC: Getting balance...");
        self.call_rpc("get_balance", GetBalanceParams::default()).await
    }

    /// Gets the primary address for a specific account index (usually 0).
    /// RPC Method: `get_address`
    pub async fn get_address(&self, account_index: u32) -> Result<MoneroAddress, ExchangeError> {
        log::debug!("Monero RPC: Getting address for account index {}...", account_index);
        let params = GetAddressParams { account_index };
         // The response structure for get_address usually includes the address directly
         // or a list of addresses if subaddresses were requested. We need to adjust parsing.
         // Assuming the response is directly `{"address": "...", "addresses": [...]}` within the `result` field.
         #[derive(Deserialize)]
         struct GetAddressResult { address: String } // Simplified - adjust based on actual RPC response
         let result: GetAddressResult = self.call_rpc("get_address", params).await?;
         Ok(MoneroAddress { address: result.address, address_index: None }) // Assuming primary address return
    }

    // TODO: Implement get_subaddress method using `get_address` with specific indices if needed.

    /// Initiates a transfer to one or more destinations.
    /// RPC Method: `transfer`
    pub async fn transfer(
        &self,
        destinations: Vec<Destination>, // Already expects amount in atomic units
        payment_id: Option<String>, // Optional hex payment ID
        priority: Option<u32>, // 0=default, 1=unimportant, 2=normal, 3=elevated
        mixin: Option<u32>, // Ringsize (defaults usually best)
    ) -> Result<MoneroTransferResult, ExchangeError> {
        log::info!("Monero RPC: Initiating transfer to {} destinations...", destinations.len());

        // Validate destination addresses if monero feature enabled
        #[cfg(feature = "monero_support")]
        for dest in &destinations {
             Address::from_str(&dest.address).map_err(|e| {
                 ExchangeError::InvalidInput(format!("Invalid destination address '{}': {}", dest.address, e))
             })?;
        }

        let params = TransferParams {
            destinations,
            account_index: Some(0), // Default to primary account
            priority,
            mixin,
            unlock_time: Some(0), // Default: no lock time
            payment_id,
            get_tx_key: Some(true), // Request tx key to prove payment
            get_tx_hex: Some(false), // Don't need raw hex usually
        };
        self.call_rpc("transfer", params).await
    }

     // TODO: Add other useful RPC methods:
     // - `get_transfers`: To list recent transactions
     // - `create_wallet` / `open_wallet`: If managing wallet files via RPC
     // - `validate_address`: To check address validity
     // - `make_integrated_address`: To create addresses with embedded payment IDs
}