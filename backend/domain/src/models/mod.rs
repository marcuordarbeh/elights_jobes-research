// /home/inno/elights_jobes-research/backend/domain/src/models/mod.rs
pub mod user;
pub mod wallet; // Renamed from account
pub mod transaction;
pub mod audit_log; // Added audit log model

// Re-export main models and enums for easier access
pub use user::{User, NewUser, UpdateUser};
pub use wallet::{Wallet, NewWallet, UpdateWallet, WalletType, WalletStatus};
pub use transaction::{
    Transaction, NewTransaction, UpdateTransaction, TransactionType, TransactionStatus,
    PaymentDetails, CardDetails, AchDetails, WireDetails, CheckDetails, CryptoDetails
};
pub use audit_log::{AuditLog, NewAuditLog, AuditOutcome, AuditTargetType};