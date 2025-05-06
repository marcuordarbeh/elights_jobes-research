// /home/inno/elights_jobes-research/backend/domain/src/crypto/mod.rs

// Advanced Cryptography Concepts (Placeholders)
pub mod blindae;      // Blind Attribute-Based Encryption concepts
pub mod zk_proofs;    // Zero-Knowledge Proof concepts

// Core Wallet Logic (Internal Representation, if distinct from exchange client)
// pub mod wallet_core; // Optional: Could be merged into models/wallet if simple

// Utility Functions
pub mod utils;        // Basic crypto math utils (e.g., unit conversions)

// Re-export key functions or types if needed
pub use utils::{btc_to_satoshis, satoshis_to_btc, xmr_to_atomic_units, atomic_units_to_xmr};