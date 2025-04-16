// domain/crypto/utils.rs

/// Provides utility functions for crypto operations.

/// Converts a string amount to satoshis (dummy conversion).
pub fn convert_to_satoshis(amount: f64) -> u64 {
    (amount * 100_000_000.0) as u64
}

/// Converts a string amount to atomic units for Monero (dummy conversion).
pub fn convert_to_atomic_units(amount: f64) -> u64 {
    (amount * 1_000_000_000_000.0) as u64
}
