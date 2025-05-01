// /home/inno/elights_jobes-research/bank-integrations/src/simulators/test_bank_simulators.rs
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;
use tokio::time::sleep; // Use tokio sleep for async compatibility
use rust_decimal::Decimal;

#[derive(Error, Debug)]
pub enum SimulatorError {
    #[error("Invalid amount: must be positive")]
    InvalidAmount,
    #[error("Simulated network timeout")]
    Timeout,
    #[error("Simulated insufficient funds")]
    InsufficientFunds,
    #[error("Simulated bank system error")]
    SystemError,
    #[error("Unknown simulation error")]
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SimulatedResponse {
    pub success: bool,
    pub bank: String,
    pub transaction_id: Option<String>,
    pub message: String,
}

/// Simulates a bank transaction processing with random delays and outcomes.
pub async fn simulate_transaction(
    bank: &str,
    amount: Decimal,
    simulate_failure_rate: f64, // e.g., 0.1 for 10% failure chance
) -> Result<SimulatedResponse, SimulatorError> {
    if amount <= Decimal::ZERO {
        return Err(SimulatorError::InvalidAmount);
    }

    // Simulate variable network delay
    let delay_ms = rand::thread_rng().gen_range(50..1500);
    sleep(Duration::from_millis(delay_ms)).await;

    // Simulate potential failures
    let mut rng = rand::thread_rng();
    if rng.gen_bool(simulate_failure_rate) {
        // Simulate different failure types
        let failure_type = rng.gen_range(0..3);
        return match failure_type {
            0 => Err(SimulatorError::Timeout),
            1 => Err(SimulatorError::InsufficientFunds),
            _ => Err(SimulatorError::SystemError),
        };
    }

    // Simulate success
    let txn_id = format!("SIM_TX_{}", rand::random::<u64>());
    Ok(SimulatedResponse {
        success: true,
        bank: bank.to_string(),
        transaction_id: Some(txn_id),
        message: format!("Successfully simulated ${:.2} transaction with {}", amount, bank),
    })
}