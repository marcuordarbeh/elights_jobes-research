// /home/inno/elights_jobes-research/backend/domain/src/payments/mod.rs

pub mod ach;
pub mod card;
pub mod check; // If supporting check processing
pub mod generator;
pub mod iso20022;
pub mod validator;
pub mod wire;
pub mod payment_processor; // Added module for processing logic

// Re-export key functions or structs if needed
pub use ach::process_ach_payment;
pub use card::process_card_payment;
pub use check::process_check_payment;
pub use wire::process_wire_payment;
pub use validator::{validate_ach_details, validate_swift_bic, validate_iban}; // Updated validator names
pub use iso20022::build_iso20022_message;
pub use generator::{generate_routing_number, generate_account_number, generate_bank_name};
pub use payment_processor::PaymentProcessor;