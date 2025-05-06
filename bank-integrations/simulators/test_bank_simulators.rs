// /home/inno/elights_jobes-research/bank-integrations/src/simulators/test_bank_simulators.rs
use crate::error::BankClientError; // Use BankClientError potentially
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;
use tokio::time::sleep;
use rust_decimal::Decimal;
use serde_json::Value as JsonValue;

// Renaming error to avoid conflict if used elsewhere
#[derive(Error, Debug, Clone)]
pub enum SimulatorError {
    #[error("Simulated network timeout")]
    Timeout,
    #[error("Simulated insufficient funds")]
    InsufficientFunds,
    #[error("Simulated invalid request data")]
    InvalidRequest,
    #[error("Simulated bank system error")]
    SystemError,
    #[error("Unknown simulation error")]
    Unknown,
}

/// Generic response from the bank simulator.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SimulatedBankResponse {
    pub success: bool,
    pub bank_name: String,
    pub reference_id: Option<String>,
    pub message: String,
    pub data: Option<JsonValue>, // Include simulated data (e.g., account info, status)
}

/// Simulates various bank API calls with delays and potential failures.
pub async fn simulate_bank_api_call(
    bank_name: &str,
    operation: &str, // e.g., "fetch_balance", "initiate_payment"
    // input_data: Option<&JsonValue>, // Optional input data for more complex sims
    simulate_failure_rate: f64, // e.g., 0.1 for 10% failure chance
) -> Result<SimulatedBankResponse, SimulatorError> {

    log::debug!("Simulating '{}' operation for bank '{}'", operation, bank_name);

    // Simulate variable network delay
    let delay_ms = rand::thread_rng().gen_range(50..800);
    sleep(Duration::from_millis(delay_ms)).await;

    // Simulate potential failures
    let mut rng = rand::thread_rng();
    if rng.gen_bool(simulate_failure_rate) {
        // Simulate different failure types
        let failure_type = rng.gen_range(0..4);
        log::warn!("Simulating failure for '{}' at bank '{}'", operation, bank_name);
        return match failure_type {
            0 => Err(SimulatorError::Timeout),
            1 => Err(SimulatorError::InsufficientFunds),
            2 => Err(SimulatorError::InvalidRequest),
            _ => Err(SimulatorError::SystemError),
        };
    }

    // Simulate success
    let ref_id = format!("SIM_REF_{}", rand::random::<u64>());
    let (message, data) = match operation {
         "fetch_balance" => (
             "Balance retrieved successfully".to_string(),
             Some(json!({ "currency": "USD", "available_balance": "12345.67", "ledger_balance": "12400.00"}))
         ),
         "fetch_account_info" => (
              "Account info retrieved successfully".to_string(),
              Some(json!({ "accountId": "SIM_ACC_123", "type": "Checking", "status": "Active" }))
          ),
         "initiate_payment" => (
             "Payment initiated successfully".to_string(),
             Some(json!({ "paymentId": &ref_id, "status": "Pending" }))
         ),
         "get_payment_status" => (
             "Payment status retrieved".to_string(),
              Some(json!({ "paymentId": &ref_id, "status": "Settled" }))
         ),
         _ => (format!("Operation '{}' simulated successfully", operation), None),
    };

    Ok(SimulatedBankResponse {
        success: true,
        bank_name: bank_name.to_string(),
        reference_id: Some(ref_id),
        message,
        data,
    })
}

use serde_json::json; // Ensure this is imported