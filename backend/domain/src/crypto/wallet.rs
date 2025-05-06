// // /home/inno/elights_jobes-research/backend/domain/src/crypto/wallet.rs
// use serde::{Deserialize, Serialize};
// use thiserror::Error;
// use uuid::Uuid; // For unique IDs

// #[derive(Debug, Error)]
// pub enum WalletError {
//     #[error("Failed to generate keys: {0}")]
//     KeyGeneration(String),
//     #[error("Secure key storage unavailable: {0}")]
//     KeyStorage(String),
//     #[error("Signing failed: {0}")]
//     SigningError(String),
//     #[error("Invalid operation for wallet type {0}")]
//     InvalidOperation(String),
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub enum CryptoCurrency {
//     Bitcoin,
//     Monero,
//     // Add others as needed
// }

// /// Represents a generic crypto wallet interface.
// /// Specific implementations would exist for BTC, XMR, etc.
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Wallet {
//     pub id: Uuid, // Unique identifier for the wallet
//     pub user_id: String, // Link to the user who owns the wallet
//     pub currency: CryptoCurrency,
//     pub address: String, // Public address
//     // Private key material MUST NOT be stored directly here in production.
//     // It should be managed via secure enclaves, HSMs, or external signing services.
//     #[serde(skip_serializing, skip_deserializing)] // Ensure private key is never serialized
//     private_key_ref: Option<String>, // Placeholder for reference to securely stored key
// }

// /// Creates a new wallet (dummy implementation).
// /// In practice, this would involve secure key generation and storage.
// pub fn create_wallet(user_id: String, currency: CryptoCurrency) -> Result<Wallet, WalletError> {
//     // In practice, call secure key generation routines (e.g., from Monero lib[cite: 4], Bitcoin lib)
//     // and store keys securely, returning only the public address and a reference.
//     let dummy_address = match currency {
//         CryptoCurrency::Bitcoin => format!("btc_addr_{}", rand::random::<u32>()),
//         CryptoCurrency::Monero => format!("xmr_addr_{}", rand::random::<u32>()), // Generate valid Monero address
//     };
//     println!("Generated {} wallet for user {}", match currency {
//         CryptoCurrency::Bitcoin => "BTC",
//         CryptoCurrency::Monero => "XMR",
//     }, user_id);

//     Ok(Wallet {
//         id: Uuid::new_v4(),
//         user_id,
//         currency,
//         address: dummy_address,
//         private_key_ref: Some("placeholder_key_ref".to_string()), // Store a reference, not the key
//     })
// }

// /// Signs a transaction using wallet credentials (dummy version).
// /// Real implementation requires accessing the secure key storage and using crypto libraries.
// pub fn sign_transaction(wallet: &Wallet, transaction_data: &str) -> Result<String, WalletError> {
//     if wallet.private_key_ref.is_none() {
//         return Err(WalletError::KeyStorage("Private key reference not available".to_string()));
//     }
//     // TODO: Retrieve actual private key based on private_key_ref securely
//     // TODO: Use appropriate library (Bitcoin, Monero) to sign transaction_data
//     println!(
//         "Signing transaction data '{}' with {} wallet (ID: {}) for user {}",
//         transaction_data,
//         match wallet.currency {
//             CryptoCurrency::Bitcoin => "BTC",
//             CryptoCurrency::Monero => "XMR",
//         },
//         wallet.id,
//         wallet.user_id
//     );
//     // Dummy signature
//     Ok(format!(
//         "signed({}_{})",
//         transaction_data,
//         wallet.address // Using address as part of dummy signature
//     ))
// }