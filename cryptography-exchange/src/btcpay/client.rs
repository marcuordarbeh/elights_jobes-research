// /home/inno/elights_jobes-research/cryptography-exchange/src/btcpay/client.rs
use crate::error::ExchangeError;
use crate::models::{
    CreateInvoiceRequest, InvoiceData, CreatePayoutRequest, PayoutData, WebhookInvoiceEvent,
};
use reqwest::{Client as HttpClient, Method, Response, StatusCode}; // Alias to avoid confusion
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;
use std::env;
use rust_decimal::Decimal;

#[derive(Clone)] // Make client cloneable
pub struct BTCPayClient {
    http_client: HttpClient,
    base_url: String,
    api_key: String,
    // Store ID needs to be configured or discovered
    // Option 1: Pass store_id to relevant methods
    // Option 2: Configure default store_id during client creation
    default_store_id: Option<String>,
}

impl BTCPayClient {
    /// Creates a new BTCPay Client instance from environment variables.
    pub fn new(default_store_id: Option<String>) -> Result<Self, ExchangeError> {
        dotenv::dotenv().ok(); // Load .env file

        let base_url = env::var("BTCPAY_URL")
            .map_err(|_| ExchangeError::ConfigurationError("BTCPAY_URL not set".to_string()))?;
        let api_key = env::var("BTCPAY_API_KEY")
            .map_err(|_| ExchangeError::ConfigurationError("BTCPAY_API_KEY not set".to_string()))?;

        // Configure underlying HTTP client
        let http_client = HttpClient::builder()
            .timeout(std::time::Duration::from_secs(30)) // Example timeout
            .build()
            .map_err(|e| ExchangeError::InternalError(format!("Failed to build HTTP client: {}", e)))?;

        log::info!("BTCPayClient initialized for base URL: {}", base_url);
        Ok(BTCPayClient {
            http_client,
            base_url,
            api_key,
            default_store_id,
        })
    }

    /// Helper to get the store ID for a request.
    fn get_store_id<'a>(&self, store_id_override: Option<&'a str>) -> Result<&'a str, ExchangeError> {
        store_id_override.or(self.default_store_id.as_deref()).ok_or_else(|| {
            ExchangeError::ConfigurationError(
                "BTCPay Store ID not provided and no default configured".to_string(),
            )
        })
    }

    /// Helper to build authenticated requests based on BTCPay spec (API Key or Basic Auth).
    fn build_auth_request(&self, method: Method, endpoint: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.base_url, endpoint);
        self.http_client
            .request(method, url)
            // BTCPay often uses "Authorization: token <API_KEY>" or "Authorization: Basic <base64(apiKey:)>"
            .header("Authorization", format!("token {}", self.api_key))
            .header(reqwest::header::ACCEPT, "application/json")
            .header(reqwest::header::CONTENT_TYPE, "application/json") // Needed for POST/PUT
    }

    /// Helper to handle response status and parsing.
    async fn handle_response<T: DeserializeOwned>(
        &self,
        operation_name: &str,
        response: Response,
    ) -> Result<T, ExchangeError> {
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            log::error!("BTCPay API Error ({}) - Status: {}, Body: {}", operation_name, status, body);
            // TODO: Parse BTCPay specific ProblemDetails error structure from body if possible
            return Err(ExchangeError::ApiError { status, body });
        }
        // Handle 204 No Content specifically if expected for some operations
        if status == StatusCode::NO_CONTENT {
             // This requires T to be deserializable from empty string or Option<T>
             // For simplicity, assume successful operations return JSON body or handle NO_CONTENT upstream
        }
        response.json::<T>().await.map_err(|e| {
            log::error!("Failed to parse BTCPay API response ({}): {}", operation_name, e);
            ExchangeError::ResponseParseError(e.to_string())
        })
    }

    // === API Methods based on swagger.json ===

    /// Creates a new invoice on the BTCPay Server.
    /// Corresponds to POST /api/v1/stores/{storeId}/invoices
    pub async fn create_invoice(
        &self,
        store_id: Option<&str>,
        request_payload: &CreateInvoiceRequest,
    ) -> Result<InvoiceData, ExchangeError> {
        let store_id = self.get_store_id(store_id)?;
        let endpoint = format!("/api/v1/stores/{}/invoices", store_id);
        log::debug!("BTCPayClient: Creating invoice at {}", endpoint);

        let request = self.build_auth_request(Method::POST, &endpoint)
            .json(request_payload);

        let response = request.send().await?;
        self.handle_response("CreateInvoice", response).await
    }

    /// Retrieves an existing invoice by its ID.
    /// Corresponds to GET /api/v1/stores/{storeId}/invoices/{invoiceId}
    pub async fn get_invoice(
        &self,
        store_id: Option<&str>,
        invoice_id: &str,
    ) -> Result<InvoiceData, ExchangeError> {
        let store_id = self.get_store_id(store_id)?;
        let endpoint = format!("/api/v1/stores/{}/invoices/{}", store_id, invoice_id);
        log::debug!("BTCPayClient: Getting invoice {}", invoice_id);

        let request = self.build_auth_request(Method::GET, &endpoint);
        let response = request.send().await?;
        self.handle_response("GetInvoice", response).await
    }

     /// Initiates a payout (withdrawal) from the BTCPay Server store wallet.
     /// Corresponds to POST /api/v1/stores/{storeId}/payouts
     pub async fn create_payout(
         &self,
         store_id: Option<&str>,
         request_payload: &CreatePayoutRequest,
     ) -> Result<PayoutData, ExchangeError> {
         let store_id = self.get_store_id(store_id)?;
         let endpoint = format!("/api/v1/stores/{}/payouts", store_id);
         log::debug!("BTCPayClient: Creating payout for method {}", request_payload.payment_method);

        let request = self.build_auth_request(Method::POST, &endpoint)
            .json(request_payload);

        let response = request.send().await?;
        // API returns PayoutData on success (200 OK)
        self.handle_response("CreatePayout", response).await
     }

    /// Cancels a payout that is in AwaitingApproval or AwaitingPayment state.
    /// Corresponds to DELETE /api/v1/stores/{storeId}/payouts/{payoutId}
    pub async fn cancel_payout(
        &self,
        store_id: Option<&str>,
        payout_id: &str,
    ) -> Result<(), ExchangeError> {
        let store_id = self.get_store_id(store_id)?;
        let endpoint = format!("/api/v1/stores/{}/payouts/{}", store_id, payout_id);
         log::debug!("BTCPayClient: Cancelling payout {}", payout_id);

        let request = self.build_auth_request(Method::DELETE, &endpoint);
        let response = request.send().await?;
        let status = response.status();
         if !status.is_success() {
             let body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
             log::error!("BTCPay API Error (CancelPayout) - Status: {}, Body: {}", status, body);
             return Err(ExchangeError::ApiError { status, body });
         }
         // Expect 200 OK on successful cancellation according to swagger
         Ok(())
    }

    // TODO: Add methods for other relevant BTCPay API endpoints based on swagger.json:
    // - Get Payouts: GET /api/v1/stores/{storeId}/payouts
    // - Get Store Info: GET /api/v1/stores/{storeId}
    // - Webhook Management: GET/POST/DELETE /api/v1/stores/{storeId}/webhooks/...
    // - API Key Management: GET/POST/DELETE /api/v1/api-keys/...

}