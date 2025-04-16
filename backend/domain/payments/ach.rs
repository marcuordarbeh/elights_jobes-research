// domain/payments/ach.rs

/// Processes an ACH payment. Expects a valid 9-digit routing number
/// and an account number (string format) for demonstration purposes.
pub fn process_ach_payment(amount: f64, routing: &str, account: &str) -> Result<String, &'static str> {
    if routing.len() != 9 {
        return Err("Invalid routing number; it should have 9 digits");
    }
    // For demo, generate a dummy transaction ID.
    let txn_id = format!("ACH-{}", rand::random::<u32>());
    println!(
        "Processing ACH payment of ${} using routing {} and account {}",
        amount, routing, account
    );
    Ok(txn_id)
}
