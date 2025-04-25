#[derive(Deserialize)]
pub struct BankServerConfig {
    pub host: String,
    pub port: u16,
    pub server_cert: String,
    pub server_key: String,
}

impl BankServerConfig {
    pub fn from_env() -> Self {
        BankServerConfig {
            host: std::env::var("BANK_HOST").expect("BANK_HOST unset"),
            port: std::env::var("BANK_PORT").unwrap().parse().unwrap(),
            server_cert: std::env::var("BANK_SERVER_CERT").unwrap(),
            server_key: std::env::var("BANK_SERVER_KEY").unwrap(),
        }
    }
}
