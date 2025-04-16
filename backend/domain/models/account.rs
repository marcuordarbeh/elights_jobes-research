// domain/models/account.rs

use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: i32,
    pub user_id: i32,
    pub account_number: String,
    pub routing_number: String,
    pub bank_name: String,
}
