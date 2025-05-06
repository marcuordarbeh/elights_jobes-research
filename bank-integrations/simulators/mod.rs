// /home/inno/elights_jobes-research/bank-integrations/src/simulators/mod.rs
// Simulators for testing bank interactions without live APIs

pub mod test_bank_simulators;

// Re-export simulator function and types
pub use test_bank_simulators::{simulate_bank_api_call, SimulatedBankResponse, SimulatorError};