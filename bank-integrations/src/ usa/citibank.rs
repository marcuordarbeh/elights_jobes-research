use crate::client_trait::BankClient;
use crate::error::BankClientError;
use crate::models::{AccountInfo, BankTransaction, PaymentRequest, PaymentStatus, Balance};
use async_trait::async_trait;
use reqwest::Client as HttpClient; // Alias to avoid confusion
use serde_json::json;
use std::env;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Clone)] // Make client cloneable if needed (e.g., for sharing in web server state)
pub struct ChaseClient {
    http_client: HttpClient,
    base_url: String,
    api_key: String, // Store API key securely
}

impl ChaseClient {
    /// Creates a new Chase Client instance from environment variables.
    pub fn new() -> Result<Self, BankClientError> {
        dotenv::dotenv().ok(); // Load .env file

        let base_url = env::var("CHASE_API_BASE")
            .map_err(|_| BankClientError::ConfigurationError("CHASE_API_BASE not set".to_string()))?;
        let api_key = env::var("CHASE_API_KEY")
            .map_err(|_| BankClientError::ConfigurationError("CHASE_API_KEY not set".to_string()))?;

        // Configure underlying HTTP client (timeouts, proxies etc.)
        let http_client = HttpClient::builder()
            .timeout(std::time::Duration::from_secs(30)) // Example timeout
            // TODO: Configure proxy if needed (e.g., via TOR SOCKS proxy)
            // .proxy(reqwest::Proxy::all("socks5h://tor:9050")?) // Example Tor proxy
            .build()
            .map_err(|e| BankClientError::InternalError(format!("Failed to build HTTP client: {}", e)))?;

        Ok(Self {
            http_client,
            base_url,
            api_key,
        })
    }

    // Helper to build authenticated requests
    fn build_auth_request(&self, method: reqwest::Method, endpoint: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.base_url, endpoint);
        self.http_client
            .request(method, url)
            // TODO: Determine Chase API auth scheme (API Key Header? Bearer Token?)
            // Example: API Key header
            // .header("X-API-KEY", &self.api_key)
            // Example: Bearer Token
            .bearer_auth(&self.api_key)
            .header(reqwest::header::ACCEPT, "application/json")
    }

    // Helper to handle response status and parsing
    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, BankClientError> {
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            log::error!("Chase API Error: Status={}, Body={}", status, body);
             // TODO: Parse Chase specific error structure from body if possible
            return Err(BankClientError::ApiError { status, body });
        }
        response.json::<T>().await.map_err(|e| {
            log::error!("Failed to parse Chase API response: {}", e);
            BankClientError::ResponseParseError(e.to_string())
        })
    }
}

#[async_trait]
impl BankClient for ChaseClient {
    fn bank_name(&self) -> &'static str { "JPMorgan Chase" }

    async fn fetch_account_info(&self, account_id: &str) -> Result<AccountInfo, BankClientError> {
        log::debug!("ChaseClient: Fetching account info for ID: {}", account_id);
        // TODO: Replace with actual Chase API endpoint for accounts
        let endpoint = format!("/v1/accounts/{}", account_id);
        let request = self.build_auth_request(reqwest::Method::GET, &endpoint);
        let response = request.send().await?;
        // TODO: Define actual AccountInfo struct based on Chase API response and parse it
        self.handle_response::<AccountInfo>(response).await
    }

    async fn fetch_balance(&self, account_id: &str) -> Result<Balance, BankClientError> {
        log::debug!("ChaseClient: Fetching balance for ID: {}", account_id);
        // TODO: Replace with actual Chase API endpoint for balances
        let endpoint = format!("/v1/accounts/{}/balance", account_id);
         let request = self.build_auth_request(reqwest::Method::GET, &endpoint);
        let response = request.send().await?;
        // TODO: Define actual Balance struct based on Chase API response and parse it
        self.handle_response::<Balance>(response).await
    }

    async fn list_transactions(
        &self,
        account_id: &str,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        limit: Option<u32>,
    ) -> Result<Vec<BankTransaction>, BankClientError> {
        log::debug!("ChaseClient: Listing transactions for ID: {}", account_id);
        // TODO: Replace with actual Chase API endpoint for transactions
        let endpoint = format!("/v1/accounts/{}/transactions", account_id);
        let mut request = self.build_auth_request(reqwest::Method::GET, &endpoint);
        // Add query parameters for date range and limit based on Chase API spec
        let mut query_params = Vec::new();
        if let Some(start) = start_date { query_params.push(("startDate", start.to_rfc3339())); }
        if let Some(end) = end_date { query_params.push(("endDate", end.to_rfc3339())); }
        if let Some(lim) = limit { query_params.push(("limit", lim.to_string())); } // Assuming String param
        request = request.query(&query_params);

        let response = request.send().await?;
        // TODO: Define actual BankTransaction struct based on Chase API response and parse list
        self.handle_response::<Vec<BankTransaction>>(response).await
    }

    async fn initiate_payment(&self, payment_request: &PaymentRequest) -> Result<PaymentStatus, BankClientError> {
        log::info!("ChaseClient: Initiating payment: Ref {}", payment_request.client_reference);
        // TODO: Replace with actual Chase API endpoint for initiating payments (ACH/Wire)
        let endpoint = "/v1/payments"; // Example endpoint
        // TODO: Map internal PaymentRequest to Chase API specific request body structure
        let chase_request_body = json!({
             "sourceAccountId": payment_request.debit_account_id,
             "destinationAccount": payment_request.credit_account_number,
             "destinationBankId": payment_request.credit_bank_bic, // Or routing number
             "destinationName": payment_request.credit_account_name,
             "amount": payment_request.amount.to_string(), // Send amount as string
             "currency": payment_request.currency,
             "reference": payment_request.client_reference,
             "remittance": payment_request.remittance_info,
             "paymentType": payment_request.payment_type, // e.g., "ACH", "WIRE"
             // ... other fields based on Chase API
         });

        let request = self.build_auth_request(reqwest::Method::POST, endpoint)
            .json(&chase_request_body);

        let response = request.send().await?;
        // TODO: Define actual PaymentStatus struct based on Chase API response and parse it
        self.handle_response::<PaymentStatus>(response).await
    }

    async fn get_payment_status(&self, payment_id: &str) -> Result<PaymentStatus, BankClientError> {
         log::debug!("ChaseClient: Getting payment status for ID: {}", payment_id);
         // TODO: Replace with actual Chase API endpoint for payment status
         let endpoint = format!("/v1/payments/{}", payment_id);
         let request = self.build_auth_request(reqwest::Method::GET, &endpoint);
         let response = request.send().await?;
         // TODO: Define actual PaymentStatus struct based on Chase API response and parse it
         self.handle_response::<PaymentStatus>(response).await
    }
}