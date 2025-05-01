// /home/inno/elights_jobes-research/backend/domain/src/payments/card.rs
use rust_decimal::Decimal;
use crate::error::DomainError;

/// Processes a card transaction (credit/debit/virtual)
/// This is a stub that would be extended to call a PCI DSS compliant processor[cite: 10772, 11071].
pub fn process_card_payment(
    card_number: &str,
    expiry_month: u8, // Use separate month/year
    expiry_year: u16, // Use 4-digit year
    cvv: &str,
    amount: Decimal,
) -> Result<String, DomainError> {
    // Basic Luhn check or length validation [cite: 10773]
    if card_number.len() < 13 || card_number.len() > 19 || !luhn_check(card_number) {
         return Err(DomainError::Validation("Invalid card number".to_string()));
    }
    // Basic expiry validation (more complex logic needed for real validation)
    // This needs chrono::Datelike
    // let current_year = chrono::Utc::now().year() as u16;
    // let current_month = chrono::Utc::now().month() as u8;
    // if expiry_year < current_year || (expiry_year == current_year && expiry_month < current_month) {
    //     return Err(DomainError::Validation("Card expired".to_string()));
    // }
    if expiry_month == 0 || expiry_month > 12 {
         return Err(DomainError::Validation("Invalid expiry month".to_string()));
    }
    // Basic CVV validation
    if cvv.len() < 3 || cvv.len() > 4 || !cvv.chars().all(|c| c.is_digit(10)) {
        return Err(DomainError::Validation("Invalid CVV".to_string()));
    }

     if amount <= Decimal::ZERO {
        return Err(DomainError::Validation("Amount must be positive".to_string()));
    }

    let txn_id = format!("CARD-{}", rand::random::<u32>()); // [cite: 10774]

    println!(
        "Processing card payment of {} for card ending in {}",
        amount,
        &card_number[card_number.len().saturating_sub(4)..] // [cite: 10774]
    );

    // TODO: Implement actual card processing via a payment gateway (Stripe, Adyen, etc.)
    // This requires secure handling of card details and adherence to PCI DSS.

    Ok(txn_id) // [cite: 10775]
}

// Basic Luhn algorithm check (example implementation)
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
        } else {
            return false; // Not a digit
        }
    }
    sum % 10 == 0
}