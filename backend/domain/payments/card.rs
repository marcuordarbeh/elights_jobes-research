// domain/payments/card.rs

/// Processes a card transaction (credit/debit/virtual)
/// This is a stub that would be extended to call a PCI DSS compliant processor.
pub fn process_card_payment(card_number: &str, expiry: &str, cvv: &str, amount: f64) -> Result<String, &'static str> {
    if card_number.len() < 13 || card_number.len() > 19 {
        return Err("Invalid card number length");
    }
    let txn_id = format!("CARD-{}", rand::random::<u32>());
    println!(
        "Processing card payment of ${} for card ending in {}",
        amount,
        &card_number[card_number.len()-4..]
    );
    Ok(txn_id)
}
