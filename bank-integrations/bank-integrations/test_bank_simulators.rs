// bank-integrations/test_bank_simulators.rs

use rand::Rng;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::time::{sleep, Duration};

#[derive(Debug, Error)]
pub enum SimulatorError {
    #[error("Invalid amount: must be > 0")]
    InvalidAmount,
    #[error("Network timeout")]
    Timeout,
    #[error("Unknown error occurred")]
    Unknown,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimulatedResponse {
    pub success: bool,
    pub bank: String,
    pub message: String,
}

pub async fn simulate_transaction(bank: &str, amount: f64) -> Result<SimulatedResponse, SimulatorError> {
    if amount <= 0.0 {
        return Err(SimulatorError::InvalidAmount);
    }

    // Simulate variable network delay
    let delay = rand::thread_rng().gen_range(100..1000);
    sleep(Duration::from_millis(delay)).await;

    // Randomly simulate a timeout
    if rand::thread_rng().gen_bool(0.05) {
        return Err(SimulatorError::Timeout);
    }

    Ok(SimulatedResponse {
        success: true,
        bank: bank.to_string(),
        message: format!("Simulated ${:.2} with {}", amount, bank),
    })
}
