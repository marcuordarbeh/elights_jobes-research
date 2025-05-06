// /home/inno/elights_jobes-research/backend/domain/src/security/mod.rs
pub mod audit;       // Audit logging service
pub mod auth;        // Authentication (password, JWT) logic
pub mod oauth;       // OAuth validation logic (stub)
pub mod tls;         // TLS configuration loading
pub mod hashing;     // Generic hashing utilities (e.g., for account numbers)

// Re-export key functions/structs
pub use audit::log_db_audit_event;
pub use auth::{hash_password, verify_password, authenticate_user, validate_token, Claims, AuthToken};
pub use hashing::hash_sensitive_data;