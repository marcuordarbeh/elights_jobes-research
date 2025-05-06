// /home/inno/elights_jobes-research/backend/domain/src/payments/generator.rs
use rand::{Rng, seq::SliceRandom}; // Use rand crate
use crate::models::{AchDetails, WireDetails, BankIdentifier}; // Use domain models

// Predefined list of plausible bank names (mix of real and generic)
const BANK_NAMES: &[&str] = &[
    "JPMorgan Chase", "Bank of America", "Wells Fargo", "Citibank", "U.S. Bank", "PNC Bank",
    "BNP Paribas", "Deutsche Bank", "ING Group", "Santander", "Barclays", "HSBC Europe",
    "First National Testing Bank", "Generic Credit Union", "Simulated Savings & Loan",
    "Digital Trust Bank", "Apex Financial", "Meridian Bank Corp", "Quantum Ledger Bank",
];

// Common European country codes for IBAN generation
const EURO_COUNTRY_CODES: &[&str] = &["DE", "FR", "ES", "IT", "NL", "BE", "GB"]; // Note: GB less common for SEPA now

/// Generates a random bank name from the predefined list.
pub fn generate_random_bank_name() -> String {
    let mut rng = rand::thread_rng();
    BANK_NAMES.choose(&mut rng).unwrap_or(&"Default Test Bank").to_string()
}

/// Generates random (but plausible) ACH details.
pub fn generate_random_ach_details() -> AchDetails {
    let mut rng = rand::thread_rng();
    // Generate routing number with correct checksum
    let mut digits = [0u32; 9];
    loop {
        for i in 0..8 { // Generate first 8 digits randomly
            digits[i] = rng.gen_range(0..=9);
        }
        // Calculate checksum for the 9th digit
        let checksum_base = 3 * (digits[0] + digits[3] + digits[6])
                          + 7 * (digits[1] + digits[4] + digits[7])
                          + 1 * (digits[2] + digits[5]);
        let remainder = checksum_base % 10;
        digits[8] = if remainder == 0 { 0 } else { 10 - remainder };

        // Simple retry if first digit is somehow invalid (e.g., must be 0,1,2,3 for FedACH) - basic check
        if digits[0] <= 3 {
             break;
        }
    }
    let routing_number = digits.iter().map(|d| d.to_string()).collect::<String>();

    // Generate random account number
    let account_len = rng.gen_range(8..=17);
    let account_number = (0..account_len).map(|_| rng.gen_range(0..=9).to_string()).collect();

    AchDetails { routing_number, account_number }
}

/// Generates random (but plausible) Wire details (IBAN/BIC).
pub fn generate_random_wire_details() -> WireDetails {
    let mut rng = rand::thread_rng();

    // Generate BIC
    let bank_code: String = (0..4).map(|_| rng.gen_range(b'A'..=b'Z') as char).collect();
    let country_code: String = EURO_COUNTRY_CODES.choose(&mut rng).unwrap_or(&"DE").to_string(); // Pick Euro country
    let location_code: String = (0..2).map(|_| rng.gen_range(b'A'..=b'Z') as char).collect();
    let branch_code = if rng.gen_bool(0.3) { // Less likely to have branch code
        (0..3).map(|_| rng.gen_range(b'A'..=b'Z') as char).collect::<String>()
    } else {
        "".to_string()
    };
    let swift_bic = format!("{}{}{}{}", bank_code, country_code, location_code, branch_code);

    // Generate IBAN matching the country code
    let iban = generate_random_iban(&country_code);

    WireDetails {
        swift_bic,
        account_number: iban, // Use generated IBAN as account number
        beneficiary_name: generate_random_person_name(), // Helper function needed
        remittance_info: Some(format!("Test Payment {}", rand::random::<u32>())),
        charge_details: Some("SHA".to_string()), // Example charge detail
        purpose_code: None,
        intermediary_banks: None,
        uetr: None, // UETR generated later
        swift_messages: None,
    }
}

/// Generates a random, structurally plausible IBAN for a given country.
/// Does NOT guarantee validity, only format.
fn generate_random_iban(country_code: &str) -> String {
     let mut rng = rand::thread_rng();
     // Basic BBAN lengths (approximate)
     let bban_len = match country_code {
         "DE" => 18, "FR" => 23, "ES" => 20, "IT" => 23, "NL" => 14, "BE" => 12, "GB" => 18,
         _ => 16, // Default
     };
     // Generate random alphanumeric BBAN
     let bban: String = (0..bban_len)
         .map(|_| {
             if rng.gen_bool(0.7) { // More likely digits
                 rng.gen_range(b'0'..=b'9') as char
             } else {
                 rng.gen_range(b'A'..=b'Z') as char
             }
         })
         .collect();
     // Placeholder check digits "00" - real IBANs have calculated check digits
     format!("{}00{}", country_code, bban)
}

/// Generates a random-ish person name.
fn generate_random_person_name() -> String {
    let mut rng = rand::thread_rng();
    let first_names = ["Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace", "Heidi"];
    let last_names = ["Smith", "Jones", "Williams", "Brown", "Davis", "Miller", "Wilson", "Taylor"];
    format!("{} {}", first_names.choose(&mut rng).unwrap(), last_names.choose(&mut rng).unwrap())
}

// TODO: Add generate_random_check_details if needed.
// TODO: Add generate_random_card_details (requires more realistic generation e.g., BIN ranges).