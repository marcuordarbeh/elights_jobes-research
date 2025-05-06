// /home/inno/elights_jobes-research/backend/domain/src/models/audit_log.rs
use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::{table, sql_types::{BigInt, Timestamptz, Nullable, Uuid as DieselUuid, Varchar, Jsonb, Text}};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde_json::Value as JsonValue;

// Import schema definition (resolve path as needed)
// use schema::core_schema::audit_logs;
// TODO: Resolve schema path access. Using direct table reference for now.
table! {
    core_schema.audit_logs (log_id) {
        log_id -> BigInt,
        timestamp -> Timestamptz,
        user_id -> Nullable<DieselUuid>,
        actor_identifier -> Varchar,
        action -> Varchar,
        target_type -> Nullable<Varchar>,
        target_id -> Nullable<Varchar>,
        outcome -> Varchar,
        details -> Nullable<Jsonb>,
        error_message -> Nullable<Text>,
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum AuditOutcome {
    Success,
    Failure,
}
// TODO: Implement ToSql/FromSql for AuditOutcome if using DbEnum

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum AuditTargetType {
    User,
    Wallet,
    Transaction,
    System,
    Config,
    // Add others as needed
}
// TODO: Implement ToSql/FromSql for AuditTargetType if using DbEnum


/// Represents an audit log entry fetched from the database.
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable, Clone, PartialEq)]
#[diesel(table_name = audit_logs, primary_key(log_id))]
pub struct AuditLog {
    pub log_id: i64,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<Uuid>, // User performing the action (if applicable)
    pub actor_identifier: String, // Username, System Component, API Key ID
    pub action: String, // Verb describing the action (e.g., LOGIN, CREATE_TX)
    pub target_type: Option<String>, // Type of entity acted upon
    pub target_id: Option<String>, // ID of the entity acted upon
    pub outcome: String, // Map to AuditOutcome enum
    pub details: Option<JsonValue>, // Additional context (IP, params etc.)
    pub error_message: Option<String>, // Specific error if outcome is Failure
}

/// Represents data needed to create a new audit log entry.
#[derive(Debug, Deserialize, Insertable, Clone)]
#[diesel(table_name = audit_logs)]
pub struct NewAuditLog<'a> {
    // log_id is serial, timestamp is defaulted
    pub user_id: Option<Uuid>,
    pub actor_identifier: &'a str,
    pub action: &'a str,
    pub target_type: Option<&'a str>, // Store as string, map from enum
    pub target_id: Option<&'a str>,
    pub outcome: &'a str, // Store as string, map from enum
    pub details: Option<JsonValue>,
    pub error_message: Option<&'a str>,
}