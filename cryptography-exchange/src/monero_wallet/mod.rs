// /home/inno/elights_jobes-research/cryptography-exchange/src/monero_wallet/mod.rs

pub mod client;
pub mod json_rpc; // Define JSON-RPC structures

#[cfg(feature = "monero_support")] // Only export if feature enabled
pub use client::MoneroWalletRpcClient;

// Re-export common models used by client if needed
pub use crate::models::{MoneroBalance, MoneroAddress, MoneroTransferResult};