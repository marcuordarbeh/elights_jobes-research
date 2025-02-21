use reqwest::Client;
use std::error::Error;

pub async fn convert_to_monero() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let response = client.post("https://api.fixedfloat.com/convert")
        .json(&serde_json::json!({
            "from": "USD",
            "to": "XMR",
            "amount": 100.0
        }))
        .send()
        .await?;

    let response_text = response.text().await?;
    println!("Response: {}", response_text);

    Ok(())
}