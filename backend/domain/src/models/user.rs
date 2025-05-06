// /home/inno/elights_jobes-research/backend/domain/src/models/user.rs
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
// Import the schema definition. Adjust the path if schema.rs is located elsewhere.
// Assumes schema.rs is generated in `database/` and symlinked/copied to `src/schema.rs`
// Or use `crate::schema::core_schema::users` if schema module is defined in lib.rs
// pub mod schema { include!("../../database/schema.rs"); } // Temporary include if not in lib.rs
// use schema::core_schema::users;
// TODO: Resolve schema path access. Using direct table reference for now.
use diesel::{table, sql_types::*}; // Use manual table macro if schema isn't directly included

table! {
    core_schema.users (user_id) {
        user_id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

/// Represents a user entity fetched from the database.
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable, Clone, PartialEq)]
#[diesel(table_name = users, primary_key(user_id))]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)] // Never serialize password hash
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents data needed to create a new user.
#[derive(Debug, Deserialize, Insertable, Clone)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
    // user_id, created_at, updated_at are defaulted by DB
}

/// Represents data allowed for updating a user.
#[derive(Debug, Deserialize, AsChangeset, Clone)]
#[diesel(table_name = users)]
pub struct UpdateUser<'a> {
    pub email: Option<&'a str>,
    pub password_hash: Option<&'a str>,
    // username is likely not updatable
    // updated_at is handled by trigger
}