// /home/inno/elights_jobes-research/backend/domain/src/services/reporting.rs
use crate::models::{Transaction, TransactionStatus, TransactionType}; // Use domain models
use crate::error::DomainError;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::dsl::{sum, count}; // Import aggregate functions
use bigdecimal::BigDecimal; // For handling SUM results from DB Numeric

/// Structure for the generated transaction report.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionReport {
    pub generated_at: DateTime<Utc>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub total_transactions: i64,
    pub successful_transactions: i64,
    pub failed_transactions: i64,
    // Use String for Decimal sums to avoid precision issues in JSON
    pub total_volume_usd_equiv: Option<String>, // Example: Aggregate in a base currency
    // Add more fields as needed: Breakdown by type, currency, etc.
    // pub volume_by_currency: HashMap<String, String>,
}

/// Generates a transaction report for a given period using Diesel.
pub async fn generate_transaction_report(
    conn: &mut PgConnection, // Pass mutable connection
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
) -> Result<TransactionReport, DomainError> {
    use crate::schema::transactions::dsl::*; // Import DSL for transactions table

    log::info!(
        "REPORTING | Generating transaction report from {} to {}",
        start_date, end_date
    );

    // --- Perform Aggregation Queries using Diesel ---

    // Total Count
    let total_count: i64 = transactions
        .filter(created_at.ge(start_date))
        .filter(created_at.le(end_date))
        .select(count(transaction_id))
        .first(conn)?;

    // Successful Count (Completed or Settled)
     let successful_count: i64 = transactions
        .filter(created_at.ge(start_date))
        .filter(created_at.le(end_date))
        .filter(status.eq_any([
            TransactionStatus::Completed.to_string(),
            TransactionStatus::Settled.to_string()
        ])) // Filter by successful statuses
        .select(count(transaction_id))
        .first(conn)?;

     // Failed Count
     let failed_count: i64 = transactions
        .filter(created_at.ge(start_date))
        .filter(created_at.le(end_date))
        .filter(status.eq_any([
             TransactionStatus::Failed.to_string(),
             TransactionStatus::Returned.to_string(), // Consider returned as failed for some reports
             TransactionStatus::Cancelled.to_string(), // Optionally include cancelled
             TransactionStatus::Chargeback.to_string(),
             TransactionStatus::Expired.to_string(),
        ]))
        .select(count(transaction_id))
        .first(conn)?;

    // Total Volume (Example: Summing amounts - requires conversion to base currency ideally)
    // TODO: Implement proper currency conversion before summing for meaningful volume.
    // This example sums amounts directly, which is only correct if all txns are same currency.
    let total_volume_db: Option<BigDecimal> = transactions // Diesel SUM returns Option<Numeric> mapped to Option<BigDecimal>
        .filter(created_at.ge(start_date))
        .filter(created_at.le(end_date))
        .filter(status.eq_any([ // Only sum successful transactions
            TransactionStatus::Completed.to_string(),
            TransactionStatus::Settled.to_string()
        ]))
        // .filter(currency_code.eq("USD")) // Example: filter by a single currency for simple sum
        .select(sum(amount))
        .first(conn)?;

    // Convert BigDecimal sum to Decimal string
    let total_volume_str = total_volume_db.map(|bd| {
         crate::utils::bigdecimal_to_decimal(bd).to_string() // Use conversion helper
    });


    // TODO: Add more complex aggregations (e.g., GROUP BY currency_code or transaction_type)

    let report = TransactionReport {
        generated_at: Utc::now(),
        start_date,
        end_date,
        total_transactions: total_count,
        successful_transactions: successful_count,
        failed_transactions: failed_count,
        total_volume_usd_equiv: total_volume_str, // Store as string
    };

    log::info!("REPORTING | Report generated successfully.");
    Ok(report)
}