// domain/payments/wire.rs

/// Processes a wire transfer payment.
/// Accepts a SWIFT code and account number.
pub fn process_wire_payment(amount: f64, swift: &str, account: &str) -> Result<String, &'static str> {
    if swift.len() < 8 {
        return Err("Invalid SWIFT code; expected at least 8 characters");
    }
    let txn_id = format!("WIRE-{}", rand::random::<u32>());
    println!(
        "Processing Wire payment of ${} with SWIFT {} for account {}",
        amount, swift, account
    );
    Ok(txn_id)
}
