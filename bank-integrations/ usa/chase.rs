// bank-integrations/usa/chase.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChaseError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(&'static str),
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Unexpected status code: {0}")]
    StatusError(reqwest::StatusCode),
    #[error("Failed to deserialize response")]
    ParseError,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomerInfo {
    pub customer_id: String,
    pub name: String,
    pub email: String,
}

pub struct ChaseClient {
    base_url: String,
    api_key: String,
    http: Client,
}

impl ChaseClient {
    pub fn new() -> Result<Self, ChaseError> {
        let base_url = env::var("CHASE_API_BASE").map_err(|_| ChaseError::MissingEnv("CHASE_API_BASE"))?;
        let api_key = env::var("CHASE_API_KEY").map_err(|_| ChaseError::MissingEnv("CHASE_API_KEY"))?;
        Ok(Self {
            base_url,
            api_key,
            http: Client::new(),
        })
    }

    pub async fn fetch_customer_info(&self, customer_id: &str) -> Result<CustomerInfo, ChaseError> {
        let url = format!("{}/customers/{}", self.base_url, customer_id);
        let resp = self
            .http
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(ChaseError::StatusError(resp.status()));
        }

        let customer = resp.json::<CustomerInfo>().await.map_err(|_| ChaseError::ParseError)?;
        Ok(customer)
    }
}
