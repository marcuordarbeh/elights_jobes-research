// domain/models/transaction.rs

use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i32,
    pub account_id: i32,
    pub amount: f64,
    pub currency: String,
    pub transaction_type: String, // e.g., "ACH", "wire", etc.
    pub status: String,
}
