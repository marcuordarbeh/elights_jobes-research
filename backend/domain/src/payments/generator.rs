// /home/inno/elights_jobes-research/backend/domain/src/payments/generator.rs
use rand::Rng; // [cite: 10762]

/// Generates a random (but structurally plausible) 9-digit US routing number. [cite: 10763]
/// Note: Real routing numbers are assigned, not random. Use for testing only.
pub fn generate_routing_number() -> String {
    let mut rng = rand::thread_rng();
    // Simple 9 digits - real validation includes checksum.
    (0..9).map(|_| rng.gen_range(0..=9).to_string()).collect() // [cite: 10764]
}

/// Generates a random account number as a string (length between 8 and 17 digits). [cite: 10765]
/// Note: Real account numbers have specific bank formats. Use for testing only.
pub fn generate_account_number() -> String {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(8..=17); // Common range for US accounts
    (0..len).map(|_| rng.gen_range(0..=9).to_string()).collect() // [cite: 10766]
}

/// Generates a random bank name from a predefined list. [cite: 10767]
/// Use for testing or anonymization only[cite: 11083].
pub fn generate_bank_name() -> String {
    // Extended list including provided examples [cite: 10767, 10842]
    let bank_names = vec![
        "JPMorgan Chase", "Bank of America", "Wells Fargo", "Citibank",
        "U.S. Bank", "PNC Bank", "BNP Paribas", "Deutsche Bank",
        "ING Group", "Santander", "Barclays", "HSBC Europe",
        "Bank of Testing", "Generic National Bank", "Fictional Credit Union",
    ];
    let idx = rand::thread_rng().gen_range(0..bank_names.len());
    bank_names[idx].to_string() // [cite: 10768]
}

/// Generates a structurally plausible (but likely invalid) IBAN for testing.
pub fn generate_iban(country_code: &str) -> String {
    let mut rng = rand::thread_rng();
    // Very basic format: Country Code + 2 Check Digits + Basic Bank Account Number (BBAN - length varies)
    let bban_len = match country_code {
        "DE" => 18, // Germany example
        "GB" => 18, // UK example
        "FR" => 23, // France example
        _ => 16, // Default length
    };
    let bban: String = (0..bban_len).map(|_| rng.gen_range(0..=9).to_string()).collect();
    format!("{}00{}", country_code.to_uppercase(), bban) // Using "00" as placeholder check digits
}

/// Generates a structurally plausible (but likely invalid) SWIFT/BIC for testing.
pub fn generate_swift_bic() -> String {
    let mut rng = rand::thread_rng();
    let bank_code: String = (0..4).map(|_| rng.gen_range(b'A'..=b'Z') as char).collect();
    let country_code: String = (0..2).map(|_| rng.gen_range(b'A'..=b'Z') as char).collect();
    let location_code: String = (0..2).map(|_| rng.gen_range(b'A'..=b'Z') as char).collect(); // Or digit
    // Optional branch code
    let branch_code = if rng.gen_bool(0.5) {
        (0..3).map(|_| rng.gen_range(b'A'..=b'Z') as char).collect::<String>() // Or digit
    } else {
        "".to_string()
    };
    format!("{}{}{}{}", bank_code, country_code, location_code, branch_code)
}