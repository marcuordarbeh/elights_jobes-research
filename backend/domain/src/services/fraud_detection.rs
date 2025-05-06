// /home/inno/elights_jobes-research/backend/domain/src/services/fraud_detection.rs
use crate::models::{Transaction, User, Wallet}; // Use domain models
use crate::error::DomainError;
use rust_decimal::Decimal;
use std::net::IpAddr;
use diesel::prelude::*; // If querying DB for history

// Configuration for fraud detection rules (load from config)
#[derive(Debug, Clone)]
pub struct FraudConfig {
    pub high_value_threshold_usd: Decimal, // Example threshold in USD equivalent
    pub velocity_limit_count: u32,      // Max transactions per period
    pub velocity_limit_period_secs: i64, // Period in seconds
    pub new_beneficiary_risk_score: u32, // Score added for new beneficiaries
    pub ip_risk_threshold_score: u32, // Score from IP risk service to flag
    pub final_fraud_score_threshold: u32, // Score above which transaction is flagged
}

/// Contextual information for fraud detection.
#[derive(Debug)]
pub struct FraudDetectionContext<'a> {
    pub transaction: &'a Transaction,
    pub source_wallet: Option<&'a Wallet>,
    pub destination_wallet: Option<&'a Wallet>, // Internal destination
    pub user: Option<&'a User>,
    pub source_ip: Option<IpAddr>, // IP address of the requester
    // db_connection: &'a mut PgConnection, // Pass DB connection for history checks
}

/// Performs fraud detection checks based on rules and context.
/// Returns a score and a list of reasons if flagged.
pub async fn run_fraud_checks(
    conn: &mut PgConnection, // Pass DB connection
    context: &FraudDetectionContext<'_>,
    config: &FraudConfig,
) -> Result<(u32, Vec<String>), DomainError> { // Returns (score, reasons)
    let mut total_score = 0;
    let mut reasons = Vec::new();

    log::debug!("Running fraud checks for Tx ID: {}", context.transaction.transaction_id);

    // --- Rule 1: High Value Transaction ---
    // TODO: Convert transaction amount to base currency (e.g., USD) for consistent threshold check
    let amount_usd_equiv = context.transaction.amount; // Placeholder: use actual conversion rate
    if amount_usd_equiv > config.high_value_threshold_usd {
        total_score += 50; // Example score
        reasons.push(format!("High transaction amount: {} {}", context.transaction.amount, context.transaction.currency_code));
    }

    // --- Rule 2: Transaction Velocity (Requires DB Query) ---
    if let Some(wallet_id) = context.transaction.debit_wallet_id { // Check for outbound payments
        use crate::schema::transactions::dsl::*; // Import DSL for transactions table
        use diesel::dsl::count;

        let lookback_time = Utc::now() - chrono::Duration::seconds(config.velocity_limit_period_secs);

        let recent_tx_count: i64 = transactions
            .filter(debit_wallet_id.eq(wallet_id))
            .filter(created_at.gt(lookback_time))
            .select(count(transaction_id))
            .first(conn)?;

        if recent_tx_count >= config.velocity_limit_count as i64 {
            total_score += 70; // High score for exceeding velocity
            reasons.push(format!("Transaction velocity exceeded ({} txns in {}s)", recent_tx_count, config.velocity_limit_period_secs));
        }
    }

    // --- Rule 3: New Beneficiary (Requires History Check - Conceptual) ---
    // TODO: Implement logic to check if the beneficiary (wallet_id, crypto address, bank details) is new for this user/source_wallet.
    let is_new_beneficiary = check_if_beneficiary_is_new(conn, context).await?;
    if is_new_beneficiary {
        total_score += config.new_beneficiary_risk_score;
        reasons.push("Transaction to potentially new beneficiary".to_string());
    }

    // --- Rule 4: IP Address Risk (Requires External Service) ---
    if let Some(ip) = context.source_ip {
        // TODO: Call an external IP risk scoring service (e.g., MaxMind GeoIP/minFraud, IPQualityScore)
        let ip_risk_score = query_ip_risk_service(ip).await?;
        if ip_risk_score >= config.ip_risk_threshold_score {
             total_score += ip_risk_score; // Add score from service
             reasons.push(format!("High risk IP address detected ({})", ip));
        }
    }

    // --- Rule 5: Specific Payment Type/Destination Rules ---
    // TODO: Add rules based on destination country, payment type patterns etc.

    // --- Final Decision ---
    if total_score >= config.final_fraud_score_threshold {
        log::warn!("FRAUD DETECTED (Score: {}) | TxID: {} | Reasons: {:?}",
            total_score, context.transaction.transaction_id, reasons);
    } else {
         log::info!("Fraud Check Passed (Score: {}) | TxID: {}", total_score, context.transaction.transaction_id);
    }

    Ok((total_score, reasons))
}


// --- Placeholder Helper Functions ---

/// Simulates checking if a beneficiary is new for the user/source wallet.
async fn check_if_beneficiary_is_new(
     _conn: &mut PgConnection,
     _context: &FraudDetectionContext<'_>
 ) -> Result<bool, DomainError> {
    // TODO: Implement DB query to check transaction history for the destination details
    // associated with the source user/wallet within a certain timeframe.
    Ok(rand::random::<bool>()) // Dummy result
}

/// Simulates querying an external IP risk scoring service.
async fn query_ip_risk_service(_ip: IpAddr) -> Result<u32, DomainError> {
    // TODO: Implement API call to IP risk service.
    Ok(rand::thread_rng().gen_range(0..50)) // Dummy score
}

use chrono::Utc; // Ensure Utc is imported
use rand::Rng;