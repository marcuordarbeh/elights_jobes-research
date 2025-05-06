// /home/inno/elights_jobes-research/bank-integrations/src/lib.rs

pub mod usa;
pub mod europe;
pub mod simulators; // For test_bank_simulators.rs
pub mod client_trait; // Define the common trait
pub mod models;       // Define common request/response models
pub mod error;        // Define common error type

// Re-export key components
pub use client_trait::BankClient;
pub use error::BankClientError;
pub use models::*; // Re-export common models