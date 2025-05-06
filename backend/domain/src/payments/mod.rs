// /home/inno/elights_jobes-research/backend/domain/src/payments/mod.rs

// --- Payment Type Specific Logic ---
pub mod ach;
pub mod card;
pub mod check;
pub mod wire;

// --- Standards & Formatting ---
pub mod iso20022; // ISO 20022 message generation/parsing stubs
pub mod swift_mt; // SWIFT MT message formatting stubs
pub mod rtgs; // RTGS interaction logic/concepts

// --- Core Processing & Utilities ---
pub mod validator; // Validation functions for payment details
pub mod generator; // Generation of random data for testing/dev
pub mod payment_processor; // Central payment orchestration service
pub mod gateway; // Trait/interface for external payment gateways (cards, etc.)

// Re-export key structs and functions for easier access from core-api or other modules
pub use ach::{process_ach_debit, process_ach_credit, generate_ach_file}; // Example exports
pub use card::{process_card_authorization, process_card_capture, process_card_refund};
pub use check::process_check_deposit;
pub use wire::{process_wire_transfer_outbound, process_wire_transfer_inbound};
pub use validator::{validate_payment_details, ValidationContext};
pub use generator::{
    generate_random_ach_details, generate_random_wire_details, generate_random_bank_name
};
pub use iso20022::{build_pacs_008, parse_camt_053}; // Example exports
pub use swift_mt::format_mt103;
pub use rtgs::{initiate_rtgs_payment, check_rtgs_settlement};
pub use gateway::{PaymentGateway, MockPaymentGateway}; // Export gateway trait and mock
pub use payment_processor::PaymentProcessor; // Export the orchestrator