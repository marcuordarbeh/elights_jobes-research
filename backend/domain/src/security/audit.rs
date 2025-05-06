// /home/inno/elights_jobes-research/backend/domain/src/security/audit.rs
use crate::error::DomainError;
use crate::models::{NewAuditLog, AuditOutcome, AuditTargetType}; // Use domain models
use diesel::prelude::*;
use serde_json::Value as JsonValue;
use uuid::Uuid;

/// Logs an audit event directly to the database using Diesel.
pub fn log_db_audit_event(
    conn: &mut PgConnection, // Pass mutable connection
    user_id: Option<Uuid>,
    actor_identifier: &str, // Username, System Component, API Key ID
    action: &str, // Verb describing the action (e.g., LOGIN, CREATE_TX)
    target_type: Option<AuditTargetType>, // Type of entity acted upon
    target_id: Option<&str>, // ID of the entity acted upon
    outcome: AuditOutcome, // Success or Failure
    details: Option<JsonValue>, // Additional context (IP, params etc.)
    error_message: Option<&str>, // Specific error if outcome is Failure
) -> Result<(), DomainError> {
    use crate::schema::audit_logs::dsl::*; // Import DSL for audit_logs table

    let outcome_str = match outcome {
        AuditOutcome::Success => "Success",
        AuditOutcome::Failure => "Failure",
    };

    let target_type_str = target_type.map(|t| match t { // Map enum to string if needed
        AuditTargetType::User => "User",
        AuditTargetType::Wallet => "Wallet",
        AuditTargetType::Transaction => "Transaction",
        AuditTargetType::System => "System",
        AuditTargetType::Config => "Config",
    });

    let new_log = NewAuditLog {
        // log_id is serial, timestamp is defaulted
        user_id,
        actor_identifier,
        action,
        target_type: target_type_str.as_deref(),
        target_id,
        outcome: outcome_str,
        details,
        error_message,
    };

    diesel::insert_into(audit_logs)
        .values(&new_log)
        .execute(conn) // Use execute for insert without returning result
        .map_err(|e| {
            // Log the failure to insert audit log itself (e.g., to stderr or fallback log)
            eprintln!("CRITICAL: Failed to insert audit log: {}", e);
            DomainError::Database(format!("Failed to insert audit log: {}", e))
        })?;

    Ok(())
}