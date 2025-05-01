// /home/inno/elights_jobes-research/bank-integrations/src/usa/chase.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChaseError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(&'static str),
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("API returned error status: {0}")]
    ApiError(reqwest::StatusCode),
    #[error("Failed to parse API response: {0}")]
    ParseError(String),
    #[error("Chase API specific error: {0}")]
    ChaseSpecific(String), // For Chase-defined errors
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CustomerInfo {
    pub customer_id: String,
    pub name: String,
    pub email: Option<String>, // Make fields optional if API might omit them
    // Add other relevant fields from Chase API
}

#[derive(Clone)] // Clone needed if passing client around (e.g., in Actix data)
pub struct ChaseClient {
    base_url: String,
    api_key: String,
    http: Client,
}

impl ChaseClient {
    pub fn new() -> Result<Self, ChaseError> {
        let base_url = env::var("CHASE_API_BASE")
            .map_err(|_| ChaseError::MissingEnvVar("CHASE_API_BASE"))?;
        let api_key = env::var("CHASE_API_KEY")
            .map_err(|_| ChaseError::MissingEnvVar("CHASE_API_KEY"))?;

        Ok(Self {
            base_url,
            api_key,
            // Create a reusable client with potential timeouts, etc.
            http: Client::builder()
                // .timeout(std::time::Duration::from_secs(15)) // Example timeout
                .build()
                .map_err(|e| ChaseError::RequestError(e))?,
        })
    }

    /// Fetches customer information from the Chase API.
    pub async fn fetch_customer_info(&self, customer_id: &str) -> Result<CustomerInfo, ChaseError> {
        let url = format!("{}/customers/{}", self.base_url, customer_id);
        println!("ChaseClient: Fetching customer info from {}", url); // Basic logging

        let resp = self
            .http
            .get(&url)
            .bearer_auth(&self.api_key) // Assuming Bearer token auth
            // .header("x-api-key", &self.api_key) // Or API Key header
            .send()
            .await
            .map_err(ChaseError::RequestError)?;

        if !resp.status().is_success() {
            // Log or handle specific error codes from Chase API docs if available
            let status = resp.status();
            let error_body = resp.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            println!("Chase API Error: Status {}, Body: {}", status, error_body);
            return Err(ChaseError::ApiError(status));
        }

        let customer = resp
            .json::<CustomerInfo>()
            .await
            .map_err(|e| ChaseError::ParseError(e.to_string()))?;

        Ok(customer)
    }

    // TODO: Add other methods for Chase API (e.g., initiate_payment, list_accounts)
    // async fn initiate_payment(&self, ...) -> Result<PaymentStatus, ChaseError> { ... }
}