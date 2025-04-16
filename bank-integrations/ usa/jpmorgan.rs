// bank-integrations/usa/jpmorgan.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JPMorganError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Invalid account ID provided")]
    InvalidAccountId,
    #[error("Unexpected response format")]
    ResponseFormatError,
}

#[derive(Serialize, Deserialize)]
pub struct AccountInfo {
    pub account_id: String,
    pub balance: f64,
    pub currency: String,
}

pub async fn fetch_account_info(account_id: &str) -> Result<AccountInfo, JPMorganError> {
    if account_id.is_empty() {
        return Err(JPMorganError::InvalidAccountId);
    }

    let client = Client::new();
    let url = format!("https://api.jpmorgan.com/accounts/{}", account_id);
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let account_info = response.json::<AccountInfo>().await?;
        Ok(account_info)
    } else {
        Err(JPMorganError::ResponseFormatError)
    }
}
