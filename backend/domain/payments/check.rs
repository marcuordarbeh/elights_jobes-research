// domain/payments/check.rs

/// Processes a check deposit or issuance.
pub fn process_check_payment(name: &str, routing: &str, account: &str, amount: f64) -> Result<String, &'static str> {
    if routing.len() != 9 {
        return Err("Invalid routing number for check");
    }
    let txn_id = format!("CHECK-{}", rand::random::<u32>());
    println!(
        "Processing check payment for {} of ${} using routing {} and account {}",
        name, amount, routing, account
    );
    Ok(txn_id)
}
