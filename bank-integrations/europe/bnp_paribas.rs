// bank-integrations/europe/bnp_paribas.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BNPParibasError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(&'static str),
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Server returned error code: {0}")]
    StatusError(reqwest::StatusCode),
    #[error("Failed to parse JSON")]
    ParseError,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub id: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
}

pub struct BNPParibasClient {
    base_url: String,
    oauth_token: String,
    http: Client,
}

impl BNPParibasClient {
    pub fn new() -> Result<Self, BNPParibasError> {
        let base_url =
            env::var("BNP_PARIBAS_API_BASE").map_err(|_| BNPParibasError::MissingEnv("BNP_PARIBAS_API_BASE"))?;
        let oauth_token =
            env::var("BNP_PARIBAS_OAUTH_TOKEN").map_err(|_| BNPParibasError::MissingEnv("BNP_PARIBAS_OAUTH_TOKEN"))?;
        Ok(Self {
            base_url,
            oauth_token,
            http: Client::new(),
        })
    }

    pub async fn list_transactions(&self, account_id: &str) -> Result<Vec<Transaction>, BNPParibasError> {
        let url = format!("{}/accounts/{}/transactions", self.base_url, account_id);
        let resp = self
            .http
            .get(&url)
            .bearer_auth(&self.oauth_token)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(BNPParibasError::StatusError(resp.status()));
        }

        let txs = resp.json::<Vec<Transaction>>().await.map_err(|_| BNPParibasError::ParseError)?;
        Ok(txs)
    }
}

// // bank-integrations/europe/bnp_paribas.rs

// use reqwest::Client;
// use serde::{Deserialize, Serialize};
// use thiserror::Error;

// #[derive(Debug, Error)]
// pub enum BNPParibasError {
//     #[error("HTTP request failed: {0}")]
//     RequestError(#[from] reqwest::Error),
//     #[error("Invalid account number provided")]
//     InvalidAccountNumber,
//     #[error("Unexpected response format")]
//     ResponseFormatError,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Transaction {
//     pub transaction_id: String,
//     pub amount: f64,
//     pub currency: String,
//     pub status: String,
// }

// pub async fn fetch_transactions(account_number: &str) -> Result<Vec<Transaction>, BNPParibasError> {
//     if account_number.is_empty() {
//         return Err(BNPParibasError::InvalidAccountNumber);
//     }

//     let client = Client::new();
//     let url = format!("https://api.bnp.com/accounts/{}/transactions", account_number);
//     let response = client.get(&url).send().await?;

//     if response.status().is_success() {
//         let transactions = response.json::<Vec<Transaction>>().await?;
//         Ok(transactions)
//     } else {
//         Err(BNPParibasError::ResponseFormatError)
//     }
// }
