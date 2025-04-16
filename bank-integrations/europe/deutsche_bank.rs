// bank-integrations/europe/deutsche_bank.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeutscheBankError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(&'static str),
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Unexpected status code: {0}")]
    StatusError(reqwest::StatusCode),
    #[error("Failed to deserialize JSON")]
    ParseError,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountDetails {
    pub iban: String,
    pub bic: String,
    pub balance: f64,
    pub currency: String,
}

pub struct DeutscheBankClient {
    base_url: String,
    client_id: String,
    client_secret: String,
    http: Client,
}

impl DeutscheBankClient {
    pub fn new() -> Result<Self, DeutscheBankError> {
        let base_url =
            env::var("DEUTSCHE_BANK_API_BASE").map_err(|_| DeutscheBankError::MissingEnv("DEUTSCHE_BANK_API_BASE"))?;
        let client_id =
            env::var("DEUTSCHE_BANK_CLIENT_ID").map_err(|_| DeutscheBankError::MissingEnv("DEUTSCHE_BANK_CLIENT_ID"))?;
        let client_secret = env::var("DEUTSCHE_BANK_CLIENT_SECRET")
            .map_err(|_| DeutscheBankError::MissingEnv("DEUTSCHE_BANK_CLIENT_SECRET"))?;
        Ok(Self {
            base_url,
            client_id,
            client_secret,
            http: Client::new(),
        })
    }

    async fn obtain_token(&self) -> Result<String, DeutscheBankError> {
        // In practice, perform OAuth2 client_credentials flow here
        Ok(format!("{}:{}", self.client_id, self.client_secret))
    }

    pub async fn get_account_details(&self, account_id: &str) -> Result<AccountDetails, DeutscheBankError> {
        let token = self.obtain_token().await?;
        let url = format!("{}/accounts/v1/{}", self.base_url, account_id);
        let resp = self
            .http
            .get(&url)
            .bearer_auth(token)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(DeutscheBankError::StatusError(resp.status()));
        }

        let details = resp.json::<AccountDetails>().await.map_err(|_| DeutscheBankError::ParseError)?;
        Ok(details)
    }
}
