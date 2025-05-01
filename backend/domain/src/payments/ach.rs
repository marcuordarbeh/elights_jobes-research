// /home/inno/elights_jobes-research/backend/domain/src/payments/ach.rs
use rust_decimal::Decimal;
use crate::error::DomainError;
use super::validator; // Use validator module

/// Processes an ACH payment.
/// Expects a valid 9-digit routing number [cite: 10777, 5]
/// and an account number (string format) for demonstration purposes.
pub fn process_ach_payment(
    amount: Decimal,
    routing: &str,
    account: &str,
) -> Result<String, DomainError> {
    // Validate details using the validator module [cite: 10756]
    if !validator::validate_ach_details(routing, account) {
        return Err(DomainError::Validation(
            "Invalid ACH routing or account number".to_string(),
        ));
    }

    if amount <= Decimal::ZERO {
        return Err(DomainError::Validation("Amount must be positive".to_string()));
    }

    // For demo, generate a dummy transaction ID.
    // In reality, this would involve generating NACHA-compliant files [cite: 11072]
    // and interacting with an ACH processor/network.
    let txn_id = format!("ACH-{}", rand::random::<u32>()); // [cite: 10778]

    println!(
        "Processing ACH payment of {} using routing {} and account {}",
        amount, routing, account // [cite: 10779]
    );

    // TODO: Implement actual ACH file generation and submission logic
    // This might involve specific libraries or external service calls.

    Ok(txn_id) // [cite: 10780]
}