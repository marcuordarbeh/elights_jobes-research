// domain/payments/generator.rs

use rand::Rng;

/// Generates a random (but structurally plausible) 9-digit routing number.
pub fn generate_routing_number() -> String {
    let mut rng = rand::thread_rng();
    (0..9).map(|_| rng.gen_range(0..10).to_string()).collect()
}

/// Generates a random account number as a string (length between 8 and 12 digits).
pub fn generate_account_number() -> String {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(8..=12);
    (0..len).map(|_| rng.gen_range(0..10).to_string()).collect()
}

/// Generates a random bank name from a predefined list.
pub fn generate_bank_name() -> String {
    let bank_names = vec![
        "JPMorgan Chase", "Bank of America", "Wells Fargo", "Citibank",
        "U.S. Bank", "PNC Bank", "BNP Paribas", "Deutsche Bank",
        "ING Group", "Santander", "Barclays", "HSBC Europe"
    ];
    let idx = rand::thread_rng().gen_range(0..bank_names.len());
    bank_names[idx].to_string()
}
