use reqwest::{Client, Proxy};
use std::error::Error;
use serde_json::json;

pub async fn convert_to_monero(amount: f64) -> Result<String, Box<dyn Error>> {
    let proxy = Proxy::all("socks5://127.0.0.1:9050")?;
    let client = Client::builder()
        .proxy(proxy)
        .build()?;
    
    let response = client.post("https://api.fixedfloat.com/convert")
        .json(&json!({
            "from": "USD",
            "to": "XMR",
            "amount": amount
        }))
        .send()
        .await?;
    
    let response_json: serde_json::Value = response.json().await?;
    if let Some(wallet_address) = response_json.get("walletAddress").and_then(|v| v.as_str()) {
        Ok(wallet_address.to_string())
    } else {
        Err("Failed to obtain wallet address".into())
    }
}
