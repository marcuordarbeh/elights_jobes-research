// /home/inno/elights_jobes-research/backend/domain/src/payments/validator.rs
use crate::error::DomainError;
use crate::models::{AchDetails, WireDetails, CardDetails, CheckDetails, TransactionType}; // Import detail structs
use iban::iban::validate_checksum; // Use iban crate
use chrono::{Timelike, Datelike, Utc};

// Context for validation (e.g., currency might affect rules)
pub struct ValidationContext<'a> {
    pub currency: &'a str,
    // Add other contextual info if needed (e.g., user region, transaction limit tier)
}

/// Central validation function for payment details based on type.
pub fn validate_payment_details(
    payment_type: &TransactionType,
    ach: Option<&AchDetails>,
    wire: Option<&WireDetails>,
    card: Option<&CardDetails>,
    check: Option<&CheckDetails>,
    context: &ValidationContext,
) -> Result<(), DomainError> {
    match payment_type {
        TransactionType::AchCredit | TransactionType::AchDebit => {
            validate_ach_details(ach.ok_or(DomainError::Validation("Missing ACH details".to_string()))?, context)
        },
        TransactionType::WireOutbound | TransactionType::WireInbound => {
             validate_wire_details(wire.ok_or(DomainError::Validation("Missing Wire details".to_string()))?, context)
        },
        TransactionType::CardAuthorization | TransactionType::CardCapture | TransactionType::CardRefund => {
             validate_card_details(card.ok_or(DomainError::Validation("Missing Card details".to_string()))?, context)
        },
        TransactionType::CheckDeposit | TransactionType::CheckWithdrawal => {
             validate_check_details(check.ok_or(DomainError::Validation("Missing Check details".to_string()))?, context)
        },
        // Add validation calls for other types if they have specific details
        _ => Ok(()), // No specific details to validate for InternalTransfer, Crypto (handled elsewhere), etc.
    }
}


/// Validates ACH details (routing number checksum - conceptual).
pub fn validate_ach_details(details: &AchDetails, _context: &ValidationContext) -> Result<(), DomainError> {
    // Routing number validation (length and checksum)
    if details.routing_number.len() != 9 || !details.routing_number.chars().all(|c| c.is_digit(10)) {
        return Err(DomainError::Validation("Invalid ACH routing number format (must be 9 digits)".to_string()));
    }
    // ABA routing number checksum validation:
    // (3*(d1+d4+d7) + 7*(d2+d5+d8) + 1*(d3+d6+d9)) mod 10 == 0
    let digits: Vec<u32> = details.routing_number.chars().map(|c| c.to_digit(10).unwrap_or(0)).collect();
    let checksum = 3 * (digits[0] + digits[3] + digits[6])
                 + 7 * (digits[1] + digits[4] + digits[7])
                 + 1 * (digits[2] + digits[5] + digits[8]);
    if checksum % 10 != 0 {
        return Err(DomainError::Validation("Invalid ACH routing number checksum".to_string()));
    }

    // Account number validation (basic non-empty check)
    if details.account_number.is_empty() {
        return Err(DomainError::Validation("ACH account number cannot be empty".to_string()));
    }
    // TODO: Add checks against OFAC lists or other compliance databases if applicable.
    Ok(())
}

/// Validates Wire transfer details (BIC, IBAN).
pub fn validate_wire_details(details: &WireDetails, _context: &ValidationContext) -> Result<(), DomainError> {
    // SWIFT/BIC validation
    validate_swift_bic(&details.swift_bic)?;

    // IBAN validation (using the 'iban' crate)
    if !iban::is_valid(&details.account_number).unwrap_or(false) {
         // It might be a non-IBAN account number, add checks based on country/bank if possible
         log::warn!("Wire account number '{}' is not a valid IBAN format.", details.account_number);
         // If required to be IBAN for certain destinations (e.g., SEPA), enforce it here.
         // return Err(DomainError::Validation("Invalid IBAN format".to_string()));
    }

    // Beneficiary name check
     if details.beneficiary_name.trim().is_empty() {
        return Err(DomainError::Validation("Beneficiary name cannot be empty".to_string()));
     }

    // TODO: Validate purpose codes if provided.
    // TODO: Validate intermediary bank details if provided.
    Ok(())
}


/// Validates card details (Luhn checksum, expiry).
pub fn validate_card_details(details: &CardDetails, _context: &ValidationContext) -> Result<(), DomainError> {
    // Basic Luhn check
    if details.card_number.len() < 13 || details.card_number.len() > 19 || !luhn_check(&details.card_number) {
        return Err(DomainError::Validation("Invalid card number (Luhn check failed or invalid length)".to_string()));
    }

    // Basic expiry validation
    let current_year = Utc::now().year() as u16;
    let current_month = Utc::now().month() as u8;
    // Ensure year is valid (e.g., within next 20 years)
    if details.expiry_year < current_year || details.expiry_year > current_year + 20 {
        return Err(DomainError::Validation("Invalid expiry year".to_string()));
    }
    // Ensure month is valid
    if details.expiry_month == 0 || details.expiry_month > 12 {
        return Err(DomainError::Validation("Invalid expiry month".to_string()));
    }
    // Ensure card is not expired
    if details.expiry_year < current_year || (details.expiry_year == current_year && details.expiry_month < current_month) {
       return Err(DomainError::Validation("Card has expired".to_string()));
    }

    // Basic CVV validation (length)
    if details.cvv.len() < 3 || details.cvv.len() > 4 || !details.cvv.chars().all(|c| c.is_digit(10)) {
        return Err(DomainError::Validation("Invalid CVV format".to_string()));
    }
    Ok(())
}

/// Validates check details (placeholder).
pub fn validate_check_details(details: &CheckDetails, _context: &ValidationContext) -> Result<(), DomainError> {
    // Basic checks
    if details.payee_name.trim().is_empty() {
        return Err(DomainError::Validation("Check payee name cannot be empty".to_string()));
    }
    if details.routing_number.len() != 9 || !details.routing_number.chars().all(|c| c.is_digit(10)) {
         return Err(DomainError::Validation("Invalid Check routing number format".to_string()));
     }
    if details.account_number.is_empty() {
        return Err(DomainError::Validation("Check account number cannot be empty".to_string()));
    }
    // TODO: Implement MICR line validation if parsing from images.
    // TODO: Implement duplicate check detection.
    // TODO: Implement velocity/limit checks.
    Ok(())
}


/// Validates a SWIFT/BIC code (length and structure).
fn validate_swift_bic(swift_bic: &str) -> Result<(), DomainError> {
    let len = swift_bic.len();
    if !(len == 8 || len == 11) {
        return Err(DomainError::Validation(format!("Invalid SWIFT/BIC length: {}", len)));
    }
    if !swift_bic.chars().all(|c| c.is_ascii_alphanumeric() && c.is_uppercase()) {
         return Err(DomainError::Validation("SWIFT/BIC contains invalid characters".to_string()));
    }
    // TODO: Validate country code (positions 5-6) against ISO country list?
    Ok(())
}

/// Basic Luhn algorithm check implementation.
fn luhn_check(number: &str) -> bool {
    let mut sum = 0;
    let mut alternate = false;
    for c in number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            let mut d = digit;
            if alternate {
                d *= 2;
                if d > 9 {
                    d -= 9;
                }
            }
            sum += d;
            alternate = !alternate;
        } else if c.is_whitespace() || c == '-' {
             continue; // Ignore whitespace/dashes if present
        }
        else {
            return false; // Not a digit
        }
    }
    number.chars().any(|c| c.is_digit(10)) && sum % 10 == 0 // Ensure there was at least one digit
}

/// Validates liquidity or performs checks against payment system rules (T2, CHIPS, Fedwire)
/// Placeholder function - real implementation is highly complex.
pub fn validate_payment_rules(
    payment_type: &TransactionType,
    currency: &str,
    amount: Decimal,
    value_date: Option<NaiveDate>,
    // ... other relevant details like sender/receiver banks, countries etc.
) -> Result<(), DomainError> {
    // TODO: Implement actual validation logic based on specific rulesets.
    log::debug!("Performing placeholder validation for payment rules...");

    // Example Check: Cut-off times for TARGET2 (conceptual)
    if *payment_type == TransactionType::RtgsCreditTransfer && currency == "EUR" {
        let now = Utc::now();
        // TARGET2 typically closes ~17:00-18:00 CET/CEST. Check T2 docs for exact times.
        if now.hour() >= 16 { // Rough check for late afternoon CET (adjust for timezone)
             log::warn!("Potential cut-off time issue for TARGET2 payment.");
             // Depending on policy, could return Err(DomainError::PaymentProcessing("Past cut-off time".to_string()))
        }
    }

    // TODO: Check participant status in clearing systems (requires directory access).
    // TODO: Check liquidity availability (requires API call to bank or internal calculation).
    // TODO: Validate purpose codes against regulations if applicable.

    Ok(())
}
use chrono::NaiveDate;