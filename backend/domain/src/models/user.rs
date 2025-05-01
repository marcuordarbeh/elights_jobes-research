// /home/inno/elights_jobes-research/backend/domain/src/models/user.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// If using Diesel ORM, uncomment the following lines and add diesel to Cargo.toml
// use crate::schema::core_schema::users; // Assuming schema is generated here or in db crate
// use diesel::prelude::*;

// #[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable)] // Add Diesel traits
// #[diesel(table_name = users, primary_key(username))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    // #[diesel(deserialize_as = String)] // Needed if PK is Text in Diesel
    pub username: String, // Primary Key
    pub email: String,
    pub password_hash: String, // Never store plain text passwords
    pub created_at: DateTime<Utc>,
}

// If using Diesel, add Insertable struct
// #[derive(Insertable)]
// #[diesel(table_name = users)]
// pub struct NewUser<'a> {
//     pub username: &'a str,
//     pub email: &'a str,
//     pub password_hash: &'a str,
// }