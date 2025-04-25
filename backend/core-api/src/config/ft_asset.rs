use serde::Deserialize;
#[derive(Deserialize)]
pub struct FtAssetConfig {
    pub url: String,
    pub client_cert: Option<String>,
    pub client_key: Option<String>,
    pub allowed_ips: Vec<String>,
}

impl FtAssetConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok(); // load .env :contentReference[oaicite:6]{index=6}
        FtAssetConfig {
            url: std::env::var("FT_ASSET_URL").expect("FT_ASSET_URL not set"),
            client_cert: std::env::var("FT_ASSET_CLIENT_CERT").ok(),
            client_key: std::env::var("FT_ASSET_CLIENT_KEY").ok(),
            allowed_ips: std::env::var("FT_ASSET_ALLOWED_IPS")
                .unwrap_or_default()
                .split(',')
                .map(String::from)
                .collect(),
        }
    }
}
