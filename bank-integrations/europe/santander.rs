// bank-integrations/europe/santander.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SantanderError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(&'static str),
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Unexpected HTTP status: {0}")]
    StatusError(reqwest::StatusCode),
    #[error("Failed to parse JSON response")]
    ParseError,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountSummary {
    pub account_id: String,
    pub balance: f64,
    pub currency: String,
    pub status: String,
}

pub struct SantanderClient {
    base_url: String,
    token: String,
    http: Client,
}

impl SantanderClient {
    pub fn new() -> Result<Self, SantanderError> {
        let base_url =
            env::var("SANTANDER_API_BASE").map_err(|_| SantanderError::MissingEnv("SANTANDER_API_BASE"))?;
        let token =
            env::var("SANTANDER_OAUTH_TOKEN").map_err(|_| SantanderError::MissingEnv("SANTANDER_OAUTH_TOKEN"))?;
        Ok(Self {
            base_url,
            token,
            http: Client::new(),
        })
    }

    pub async fn list_accounts(&self) -> Result<Vec<AccountSummary>, SantanderError> {
        let url = format!("{}/open-banking/v3.1/accounts", self.base_url);
        let resp = self
            .http
            .get(&url)
            .bearer_auth(&self.token)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(SantanderError::StatusError(resp.status()));
        }

        let accounts = resp.json::<Vec<AccountSummary>>().await.map_err(|_| SantanderError::ParseError)?;
        Ok(accounts)
    }
}
