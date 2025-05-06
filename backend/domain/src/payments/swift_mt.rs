// /home/inno/elights_jobes-research/backend/domain/src/payments/swift_mt.rs
use crate::error::DomainError;
use crate::models::{BankIdentifier}; // Use domain models
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

// Structure holding data for an MT103 message
#[derive(Debug, Clone)]
pub struct Mt103Details {
    pub sender_reference: String, // Field 20
    pub bank_operation_code: String, // Field 23B (e.g., "CRED")
    // Add value_date, currency, amount (Field 32A)
    pub value_date: NaiveDate, // YYYYMMDD format in MT
    pub currency: String, // ISO 4217
    pub amount: Decimal,
    // Debtor (Ordering Customer - Field 50a - K or F option)
    pub debtor_option: char, // 'K' or 'F'
    pub debtor_identifier: String, // Account for K, Name/Address lines for F
    pub debtor_address_lines: Option<Vec<String>>, // Max 4 lines for F
    // Receiver's Correspondent (Field 54A) - Optional
    pub receiver_correspondent_bic: Option<String>,
    // Beneficiary Bank (Field 57A)
    pub beneficiary_bank_bic: String,
    // Beneficiary Customer (Field 59 or 59A)
    pub beneficiary_option: char, // '' (no option) or 'A'
    pub beneficiary_identifier: String, // Account for '', Name/Address for 'A'
    pub beneficiary_address_lines: Option<Vec<String>>, // Max 4 lines for A
    // Remittance Information (Field 70)
    pub remittance_info_lines: Option<Vec<String>>, // Max 4 lines
    // Details of Charges (Field 71A)
    pub details_of_charges: String, // "BEN", "OUR", "SHA"
    // Add other optional fields as needed (e.g., 13C Time Indication, 23E Instruction Code, 71F Sender Charges, 72 Sender to Receiver Info)
}

/// Formats an MT103 message string based on provided details.
/// Placeholder: Real formatting requires strict adherence to SWIFT field rules, character sets, and line lengths.
pub fn format_mt103(details: &Mt103Details, uetr: &str) -> Result<String, DomainError> {
    log::info!("Formatting MT103 message. Ref: {}", details.sender_reference);

    // Basic validation
    if details.sender_reference.len() > 16 || details.sender_reference.contains("//") || details.sender_reference.starts_with('/') || details.sender_reference.ends_with('/') {
        return Err(DomainError::Validation("Invalid Sender Reference (Field 20) format".to_string()));
    }
    if details.beneficiary_bank_bic.len() != 8 && details.beneficiary_bank_bic.len() != 11 {
        return Err(DomainError::Validation("Invalid Beneficiary Bank BIC (Field 57A) format".to_string()));
    }
    // TODO: Add more validation for all fields based on SWIFT MT103 specs.

    let mut message = String::new();

    // Basic Block 1 & 2 (Application/User Header) - Usually added by SWIFT interface/gateway
    // message.push_str("{1:...}{2:...}"); // Placeholder

    // Basic Block 3 (User Header - containing UETR in field 121)
    message.push_str(&format!("{{3:{{121:{}}}", uetr)); // Placeholder for block 3
    // TODO: Add other mandatory tags for block 3 if needed (e.g., 108 MUR)
    message.push_str("}\n"); // End Block 3

    // Block 4 (Text Block)
    message.push_str("{4:\n");

    // Field 20: Sender's Reference
    message.push_str(&format!(":20:{}\n", details.sender_reference));
    // Field 23B: Bank Operation Code
    message.push_str(&format!(":23B:{}\n", details.bank_operation_code));
    // Field 32A: Value Date, Currency, Amount
    // Format: YYMMDDCCCAMOUNT (Amount with comma decimal separator)
    let date_str = details.value_date.format("%y%m%d").to_string();
    let amount_str = format!("{:.2}", details.amount).replace('.', ","); // Comma separator
    message.push_str(&format!(":32A:{}{}{}\n", date_str, details.currency, amount_str));

    // Field 50a: Ordering Customer (Debtor)
    message.push_str(&format!(":50{}:", details.debtor_option)); // 50K or 50F
    message.push_str(&format!("{}\n", details.debtor_identifier)); // Account or First line of Name/Addr
    if let Some(lines) = &details.debtor_address_lines {
        for line in lines.iter().take(4) { // Max 4 lines
            message.push_str(&format!("{}\n", line));
        }
    }

     // Field 54A: Receiver's Correspondent (Optional)
     if let Some(bic) = &details.receiver_correspondent_bic {
         message.push_str(&format!(":54A:{}\n", bic));
     }

    // Field 57A: Beneficiary Bank
    message.push_str(&format!(":57A:{}\n", details.beneficiary_bank_bic));

    // Field 59 or 59A: Beneficiary Customer
    if details.beneficiary_option == 'A' {
         message.push_str(":59A:"); // Beneficiary Customer (Name/Address)
    } else {
         message.push_str(":59:"); // Beneficiary Customer (Account)
    }
    message.push_str(&format!("{}\n", details.beneficiary_identifier)); // Account or First line of Name/Addr
    if let Some(lines) = &details.beneficiary_address_lines {
        for line in lines.iter().take(4) { // Max 4 lines
            message.push_str(&format!("{}\n", line));
        }
    }

    // Field 70: Remittance Information (Optional)
     if let Some(lines) = &details.remittance_info_lines {
        message.push_str(":70:");
         message.push_str(&format!("{}\n", lines.get(0).map_or("", |s| s))); // First line on same line as tag
        for line in lines.iter().skip(1).take(3) { // Max 3 more lines
            message.push_str(&format!("{}\n", line));
        }
    }

    // Field 71A: Details of Charges
    message.push_str(&format!(":71A:{}\n", details.details_of_charges));

    // TODO: Add other fields (23E, 71F, 72 etc.) based on Mt103Details

    message.push_str("-}\n"); // End Block 4

    // Basic Block 5 (Trailer) - Optional, often added by SWIFT interface
    // message.push_str("{5:...}"); // Placeholder

    Ok(message)
}

// TODO: Implement functions to parse incoming MT messages if needed. Requires careful parsing logic.
// pub fn parse_mt103(message_text: &str) -> Result<WireMessageDetails, DomainError> { ... }