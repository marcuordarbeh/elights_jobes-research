// /home/inno/elights_jobes-research/backend/domain/src/payments/check.rs
use rust_decimal::Decimal;
use crate::error::DomainError;
use super::validator;

/// Processes a check deposit or issuance.
/// NOTE: Check processing is complex and heavily regulated. This is a simplified stub.
/// Real implementation depends heavily on whether it's Check 21, remote deposit capture, etc.
pub fn process_check_payment(
    payee_name: &str, // Assuming 'name' was payee
    routing: &str,
    account: &str,
    check_number: Option<&str>, // Check number might be optional depending on context
    amount: Decimal,
) -> Result<String, DomainError> {
    // Validate routing number [cite: 10769]
    if !validator::validate_routing_number(routing) { // Assumes validator has this function
        return Err(DomainError::Validation("Invalid routing number for check".to_string()));
    }
     if account.is_empty() {
         return Err(DomainError::Validation("Account number cannot be empty".to_string()));
     }
     if payee_name.is_empty() {
         return Err(DomainError::Validation("Payee name cannot be empty".to_string()));
     }
    if amount <= Decimal::ZERO {
        return Err(DomainError::Validation("Amount must be positive".to_string()));
    }

    let txn_id = format!("CHECK-{}", rand::random::<u32>()); // [cite: 10770]

    println!(
        "Processing check payment for {} of {} using routing {} and account {}",
        payee_name, amount, routing, account // [cite: 10771]
    );

    // TODO: Implement actual check processing logic.
    // This could involve image capture (Check 21), Nacha rules for ARC/BOC,
    // or integration with specific check processing services.

    Ok(txn_id)
}