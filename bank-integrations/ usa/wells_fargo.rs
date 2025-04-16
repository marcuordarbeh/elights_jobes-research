// bank-integrations/usa/wells_fargo.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WellsFargoError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(&'static str),
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Non-success status code: {0}")]
    StatusError(reqwest::StatusCode),
    #[error("Failed to parse response")]
    ParseError,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountInfo {
    pub account_id: String,
    pub available_balance: f64,
    pub ledger_balance: f64,
    pub currency: String,
}

pub struct WellsFargoClient {
    base_url: String,
    api_key: String,
    http: Client,
}

impl WellsFargoClient {
    pub fn new() -> Result<Self, WellsFargoError> {
        let base_url =
            env::var("WELLS_FARGO_API_BASE").map_err(|_| WellsFargoError::MissingEnv("WELLS_FARGO_API_BASE"))?;
        let api_key =
            env::var("WELLS_FARGO_API_KEY").map_err(|_| WellsFargoError::MissingEnv("WELLS_FARGO_API_KEY"))?;
        Ok(Self {
            base_url,
            api_key,
            http: Client::new(),
        })
    }

    pub async fn fetch_account_info(&self, account_id: &str) -> Result<AccountInfo, WellsFargoError> {
        let url = format!("{}/accounts/{}", self.base_url, account_id);
        let resp = self
            .http
            .get(&url)
            .header("x-api-key", &self.api_key)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(WellsFargoError::StatusError(resp.status()));
        }

        let info = resp.json::<AccountInfo>().await.map_err(|_| WellsFargoError::ParseError)?;
        Ok(info)
    }
}
