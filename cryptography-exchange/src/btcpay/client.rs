// /home/inno/elights_jobes-research/cryptography-exchange/src/btcpay/client.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use crate::error::ExchangeError; // Use crate-level error
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")] // Match BTCPay's common casing
pub struct InvoiceMetadata {
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,
    // Add other metadata fields as needed
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCheckoutOptions {
    // Define checkout options if needed, e.g., redirectURL
    pub redirect_url: Option<String>,
}

#[derive(Debug, Serialize)] // Request structure
#[serde(rename_all = "camelCase")]
struct CreateInvoiceRequest<'a> {
    #[serde(with = "rust_decimal::serde::str")] // Send amount as string
    amount: Decimal,
    currency: &'a str,
    metadata: Option<InvoiceMetadata>,
    checkout: Option<InvoiceCheckoutOptions>,
    // Add other fields like description, buyer info, etc.
}


#[derive(Debug, Deserialize, Clone)] // Response structure
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub id: String,
    #[serde(with = "rust_decimal::serde::str")] // Receive amount as string
    pub amount: Decimal,
    pub currency: String,
    pub status: String, // e.g., "New", "Paid", "Expired"
    pub checkout_link: Option<String>, // URL for the customer to pay
    // Add other fields like expirationTime, serverTime, etc.
}

#[derive(Clone)]
pub struct BTCPayClient {
    pub base_url: String,
    pub api_key: String,
    http: Client,
}

impl BTCPayClient {
    /// Creates a new BTCPay Client instance from environment variables.
    pub fn new() -> Result<Self, ExchangeError> {
        let base_url = env::var("BTCPAY_URL")
            .map_err(|_| ExchangeError::MissingConfig("BTCPAY_URL"))?;
        let api_key = env::var("BTCPAY_API_KEY")
            .map_err(|_| ExchangeError::MissingConfig("BTCPAY_API_KEY"))?;

        Ok(BTCPayClient {
            base_url,
            api_key,
            http: Client::new(),
        })
    }

    /// Creates a new invoice on the BTCPay Server.
    pub async fn create_invoice(
        &self,
        amount: Decimal,
        currency: &str,
        order_id: Option<String>, // Example metadata
        redirect_url: Option<String> // Example checkout option
    ) -> Result<Invoice, ExchangeError> {
        if amount <= Decimal::ZERO {
            return Err(ExchangeError::InvalidAmount("Invoice amount must be positive".to_string()));
        }

        let request_payload = CreateInvoiceRequest {
            amount,
            currency,
            metadata: order_id.map(|id| InvoiceMetadata { order_id: Some(id) }),
            checkout: redirect_url.map(|url| InvoiceCheckoutOptions { redirect_url: Some(url) })
        };

        let url = format!("{}/api/v1/stores/YOUR_STORE_ID/invoices", self.base_url); // Replace YOUR_STORE_ID
        println!("BTCPayClient: Creating invoice at {}", url);

        let resp = self.http
            .post(&url)
            // Use correct auth for BTCPay: typically 'token <API_KEY>' or 'Basic <base64(user:pass)>'
            .header("Authorization", format!("token {}", self.api_key))
            .json(&request_payload)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let error_body = resp.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            println!("BTCPay API Error: Status {}, Body: {}", status, error_body);
            return Err(ExchangeError::ApiError(status, error_body));
        }

        let invoice = resp
            .json::<Invoice>()
            .await
            .map_err(|e| ExchangeError::ParseError(e.to_string()))?;

        Ok(invoice)
    }

     // TODO: Add methods to get invoice status, list invoices, etc.
     // async fn get_invoice(&self, invoice_id: &str) -> Result<Invoice, ExchangeError> { ... }
}