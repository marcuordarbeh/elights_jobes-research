// cryptography-exchange/btcpay/client.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub amount: f64,
    pub currency: String,
}

pub struct BTCPayClient {
    pub base_url: String,
    pub api_key: String,
}

impl BTCPayClient {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        BTCPayClient {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
        }
    }

    pub async fn create_invoice(&self, amount: f64, currency: &str) -> Result<Invoice, reqwest::Error> {
        let client = Client::new();
        let invoice = Invoice {
            id: "".to_string(),
            amount,
            currency: currency.to_string(),
        };
        let res = client
            .post(&format!("{}/invoices", self.base_url))
            .header("Authorization", format!("token {}", self.api_key))
            .json(&invoice)
            .send()
            .await?
            .json::<Invoice>()
            .await?;
        Ok(res)
    }
}
