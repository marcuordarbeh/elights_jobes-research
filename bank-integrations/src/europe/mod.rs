// /home/inno/elights_jobes-research/bank-integrations/src/europe/mod.rs
// Clients for major European Banks

pub mod bnp_paribas;
pub mod deutsche_bank;
pub mod ing_group;
pub mod santander;
pub mod barclays;
pub mod hsbc_europe;

// Re-export client types
pub use bnp_paribas::BnpParibasClient;
pub use deutsche_bank::DeutscheBankClient;
pub use ing_group::IngGroupClient;
pub use santander::SantanderClient;
pub use barclays::BarclaysClient;
pub use hsbc_europe::HsbcEuropeClient;