// /home/inno/elights_jobes-research/backend/domain/src/payments/payment_processor.rs
use diesel::prelude::*;
use crate::models::{
    Transaction, TransactionType, TransactionStatus, Wallet, User,
    AchDetails, WireDetails, CardDetails, CheckDetails, PaymentDetails, UpdateTransaction, NewTransaction
};
use crate::error::DomainError;
use crate::payments::{
    ach, card, check, wire, rtgs, validator, // Import specific payment modules
    gateway::{PaymentGateway}, // Import gateway trait
};
use crate::services::fraud_detection; // Import fraud detection
use crate::security::audit; // Import audit logging
use rust_decimal::Decimal;
use uuid::Uuid;
use serde_json::json;

/// Structure holding dependencies for payment processing.
pub struct PaymentProcessor<'a> {
    // Use lifetimes if PgConnection is passed directly, or owned pool if needed long-term
    db_connection: &'a mut PgConnection,
    // Inject the specific payment gateway implementation being used
    card_gateway: &'a dyn PaymentGateway,
    // Add other dependencies like fraud service config, rate service client etc.
}

// Represents a generic payment request
#[derive(Debug)]
pub struct PaymentRequest<'a> {
    pub initiating_user_id: Uuid,
    pub amount: Decimal,
    pub currency: &'a str,
    pub payment_type: TransactionType,
    pub source_wallet_id: Option<Uuid>, // Internal source
    pub destination_wallet_id: Option<Uuid>, // Internal destination
    // External details (depending on type)
    pub ach_details: Option<&'a AchDetails>,
    pub wire_details: Option<&'a WireDetails>,
    pub card_token: Option<&'a str>, // Use token, not raw details
    pub check_details: Option<&'a CheckDetails>,
    pub crypto_address: Option<&'a str>, // For crypto withdrawals
    // Common fields
    pub description: &'a str,
    pub metadata: Option<serde_json::Value>,
}


impl<'a> PaymentProcessor<'a> {
    /// Creates a new PaymentProcessor instance.
    pub fn new(
        db_connection: &'a mut PgConnection,
        card_gateway: &'a dyn PaymentGateway,
    ) -> Self {
        PaymentProcessor { db_connection, card_gateway }
    }

    /// Processes an outbound payment request.
    pub async fn process_outbound_payment(
        &mut self,
        request: PaymentRequest<'a>,
    ) -> Result<Transaction, DomainError> {
        log::info!("Processing outbound payment request. User: {}, Type: {:?}, Amount: {} {}",
            request.initiating_user_id, request.payment_type, request.amount, request.currency);

        // Use a database transaction for atomicity
        self.db_connection.transaction(|conn| {
            // Box the future to handle different async blocks within transaction
            Box::pin(async move {
                // --- 1. Initial Validation & Wallet Checks ---
                if request.amount <= Decimal::ZERO {
                    return Err(DomainError::Validation("Amount must be positive".to_string()));
                }
                // Validate currency code (basic check)
                if iso_4217::CurrencyCode::try_from(request.currency).is_err() {
                    return Err(DomainError::Validation(format!("Invalid currency code: {}", request.currency)));
                }

                let source_wallet_id = request.source_wallet_id
                    .ok_or(DomainError::Validation("Source wallet ID required for outbound payment".to_string()))?;

                // Find source wallet, check status and currency, lock row for update
                let source_wallet: Wallet = crate::schema::wallets::table
                    .find(source_wallet_id)
                    .for_update() // Lock the row
                    .first(conn)
                    .map_err(|e| DomainError::NotFound(format!("Source wallet {} not found or lock failed: {}", source_wallet_id, e)))?;

                if source_wallet.status != crate::models::WalletStatus::Active.to_string() { // Compare string if enum not mapped
                     return Err(DomainError::Validation(format!("Source wallet {} is not active", source_wallet_id)));
                 }
                 if source_wallet.currency_code != request.currency {
                     return Err(DomainError::Validation(format!("Source wallet currency ({}) does not match transaction currency ({})", source_wallet.currency_code, request.currency)));
                 }

                // --- 2. Check Funds ---
                if source_wallet.balance < request.amount {
                    return Err(DomainError::InsufficientFunds(source_wallet_id));
                }

                // --- 3. Create Initial Transaction Record ---
                 let new_tx = NewTransaction {
                    transaction_id: None, // Let DB generate UUID
                    debit_wallet_id: Some(source_wallet_id),
                    credit_wallet_id: request.destination_wallet_id, // Can be None for external
                    transaction_type: request.payment_type.to_string().as_str(),
                    status: TransactionStatus::Pending.to_string().as_str(),
                    amount: request.amount,
                    currency_code: request.currency,
                    description: Some(request.description),
                    external_ref_id: None,
                    metadata: request.metadata.clone(), // Clone metadata
                };

                 let mut transaction: Transaction = diesel::insert_into(crate::schema::transactions::table)
                    .values(&new_tx)
                    .get_result(conn)?;

                // --- 4. Debit Source Wallet Immediately ---
                // This ensures funds are reserved before calling external systems.
                let new_balance = source_wallet.balance - request.amount;
                diesel::update(crate::schema::wallets::table.find(source_wallet_id))
                    .set(crate::schema::wallets::balance.eq(decimal_to_bigdecimal(new_balance))) // Use BigDecimal for Diesel
                    .execute(conn)?;
                log::info!("Debited {} {} from wallet {}", request.amount, request.currency, source_wallet_id);


                // --- 5. Perform Fraud Check (Conceptual) ---
                // let is_fraudulent = fraud_detection::detect_fraud(&transaction)?; // Pass immutable borrow
                // if is_fraudulent {
                //      // Rollback debit, mark transaction as failed/requires_action
                //      diesel::update(crate::schema::wallets::table.find(source_wallet_id))
                //          .set(crate::schema::wallets::balance.eq(decimal_to_bigdecimal(source_wallet.balance))) // Revert balance
                //          .execute(conn)?;
                //      transaction = diesel::update(crate::schema::transactions::table.find(transaction.transaction_id))
                //          .set(crate::schema::transactions::status.eq(TransactionStatus::Failed.to_string())) // Or RequiresAction
                //          .get_result(conn)?;
                //      return Err(DomainError::PaymentProcessing("Fraud detected".to_string()));
                // }


                // --- 6. Route to Specific Payment Logic ---
                // Note: External calls (await) cannot be made directly inside diesel::transaction closure
                // Strategy: Perform DB changes, commit, then make external call, then update DB again in new transaction.
                // Or, use transaction manager pattern if library supports it.
                // For simplicity here, assume external calls are done AFTER this DB transaction commits.

                // Update status before potential commit
                 transaction = diesel::update(crate::schema::transactions::table.find(transaction.transaction_id))
                         .set(crate::schema::transactions::status.eq(TransactionStatus::Processing.to_string())) // Mark as processing
                         .get_result(conn)?;


                // --- 7. Log Audit Event ---
                 audit::log_db_audit_event(
                     conn,
                     Some(request.initiating_user_id),
                     &request.initiating_user_id.to_string(), // Actor ID
                     "INITIATE_OUTBOUND_PAYMENT",
                     Some("TRANSACTION"),
                     Some(&transaction.transaction_id.to_string()),
                     AuditOutcome::Success,
                     Some(json!({"amount": request.amount.to_string(), "currency": request.currency, "type": request.payment_type.to_string()})),
                     None // No error message for success
                 )?;


                Ok(transaction) // Return the transaction state BEFORE external calls
            }) // End Box::pin(async move)
        })?; // End db_connection.transaction


        // --- 7. Make External Calls (AFTER DB Transaction Commit) ---
        // We use the 'transaction' state returned from the DB transaction.
        let processing_result: Result<String, DomainError> = match request.payment_type {
            TransactionType::AchCredit => {
                let details = request.ach_details
                    .ok_or(DomainError::Validation("Missing ACH details for credit".to_string()))?;
                // ACH processing might return immediately or later via webhook
                // For now, assume it just needs submission details.
                // TODO: Call ACH service/gateway here (e.g., submit to batch). Requires external call.
                 ach::process_ach_credit(self.db_connection, request.initiating_user_id, request.source_wallet_id.unwrap(), details, request.amount, request.description, request.metadata).await?;
                Ok("ACH Credit Submitted".to_string()) // Placeholder result
            },
             TransactionType::WireOutbound => {
                let details = request.wire_details
                    .ok_or(DomainError::Validation("Missing Wire details for outbound".to_string()))?;
                // TODO: Call Wire service/gateway here. Requires external call.
                 wire::process_wire_transfer_outbound(self.db_connection, request.initiating_user_id, request.source_wallet_id.unwrap(), details, request.amount, request.currency, request.description, true, request.metadata).await?;
                 Ok("Wire Transfer Submitted".to_string()) // Placeholder result
            },
             TransactionType::CardAuthorization => {
                 // Card Auth involves external call but doesn't usually move funds yet
                 // The debit might have been premature above - card flows differ.
                 // Revisit the flow: create Tx, call gateway, update Tx & MAYBE debit wallet later on capture.
                 // For now, return error indicating flow needs adjustment.
                 return Err(DomainError::NotSupported("Card Authorization flow needs adjustment in processor".to_string()));
             },
             TransactionType::CryptoBtcSend | TransactionType::CryptoXmrSend => {
                  let address = request.crypto_address
                      .ok_or(DomainError::Validation("Missing destination crypto address".to_string()))?;
                  // TODO: Call cryptography_exchange service here. Requires external call.
                  // crypto_exchange_client.send_crypto(wallet_id, address, amount, currency).await?;
                  Ok(format!("{:?} Submitted", request.payment_type))
             }
            // Add other outbound types (CheckWithdrawal?)
            _ => Err(DomainError::NotSupported(format!("Outbound processing not supported for type: {:?}", request.payment_type)))
        };

        // --- 8. Final DB Update (New Transaction) ---
        // Update status based on external call result (e.g., Submitted, Failed)
         self.db_connection.transaction(|conn| {
            Box::pin(async move {
                let final_status;
                let external_ref;
                match processing_result {
                     Ok(ext_ref) => {
                         final_status = TransactionStatus::Submitted;
                         external_ref = Some(ext_ref);
                         log::info!("External call successful for Tx: {}", transaction.transaction_id);
                     }
                     Err(e) => {
                         final_status = TransactionStatus::Failed;
                         external_ref = None;
                         log::error!("External call failed for Tx: {}: {}", transaction.transaction_id, e);
                         // TODO: CRITICAL - Need to re-credit the source wallet if debit occurred in step 4!
                         // This requires careful state management and potentially a separate reversal transaction.
                         // update_wallet_balance(conn, source_wallet_id, request.amount)?; // Re-credit
                     }
                 };

                let update_tx = UpdateTransaction {
                    status: Some(final_status.to_string().as_str()),
                    external_ref_id: external_ref.as_deref(),
                    metadata: None, // Keep existing metadata unless updated
                    settlement_at: None,
                };
                 let updated_tx = diesel::update(crate::schema::transactions::table.find(transaction.transaction_id))
                    .set(&update_tx)
                    .get_result(conn)?;

                Ok(updated_tx)
            }) // End Box::pin
        }) // End transaction
    }


    /// Handles updates for a payment (e.g., from webhooks, settlement confirmations).
    pub async fn update_payment_status(
        &mut self,
        transaction_id: Uuid,
        new_status: TransactionStatus,
        external_ref: Option<&str>,
        settlement_time: Option<DateTime<Utc>>,
        metadata_update: Option<serde_json::Value>, // For adding failure reasons, etc.
    ) -> Result<Transaction, DomainError> {
         log::info!("Updating status for Tx: {} to {:?}", transaction_id, new_status);

         self.db_connection.transaction(|conn| {
             Box::pin(async move {
                 let mut tx: Transaction = crate::schema::transactions::table
                    .find(transaction_id)
                    .for_update() // Lock row
                    .first(conn)
                    .map_err(|e| DomainError::NotFound(format!("Transaction {} not found for update: {}", transaction_id, e)))?;

                 // TODO: Add state transition validation (e.g., cannot move from Completed to Pending)

                 let update_data = UpdateTransaction {
                     status: Some(new_status.to_string().as_str()),
                     external_ref_id: external_ref,
                     settlement_at: settlement_time,
                     metadata: metadata_update, // Replace or merge metadata logic needed
                 };

                let updated_tx = diesel::update(crate::schema::transactions::table.find(transaction_id))
                     .set(&update_data)
                     .get_result(conn)?;

                // --- Handle Financial Impact of Status Change ---
                 match new_status {
                     TransactionStatus::Completed | TransactionStatus::Settled => {
                         // If crediting an internal wallet, update balance
                         if let Some(wallet_id) = updated_tx.credit_wallet_id {
                            // TODO: Implement atomic balance update for credit side
                            // update_wallet_balance(conn, wallet_id, updated_tx.amount)?;
                             log::info!("Credited wallet {} for completed Tx {}", wallet_id, transaction_id);
                         }
                     }
                     TransactionStatus::Failed | TransactionStatus::Returned | TransactionStatus::Cancelled => {
                          // If debiting an internal wallet, need to revert the debit
                          if let Some(wallet_id) = updated_tx.debit_wallet_id {
                              // Check if it was actually debited (status might have been Pending before failure)
                              // This requires careful state tracking or checking previous status.
                              // TODO: Implement atomic balance update for debit reversal.
                              // update_wallet_balance(conn, wallet_id, updated_tx.amount)?; // Re-credit
                              log::warn!("Reversal needed for failed/returned Tx {} debit from wallet {}", transaction_id, wallet_id);
                          }
                     }
                     _ => {} // Other statuses might not have immediate financial impact
                 }

                // --- Log Audit Event ---
                 audit::log_db_audit_event(
                     conn,
                     None, // UserID might not be available for webhook updates
                     "SYSTEM", // Actor
                     "UPDATE_PAYMENT_STATUS",
                     Some("TRANSACTION"),
                     Some(&transaction_id.to_string()),
                     AuditOutcome::Success, // Assuming DB update worked
                     Some(json!({"new_status": new_status.to_string(), "external_ref": external_ref})),
                     None
                 )?;


                 Ok(updated_tx)
             }) // End Box::pin
         }) // End transaction
    }

    // Add methods for handling inbound payments, processing notifications etc.
}

// Helper function to convert Decimal to BigDecimal (required by Diesel for Numeric)
fn decimal_to_bigdecimal(d: Decimal) -> BigDecimal {
     use std::str::FromStr;
     BigDecimal::from_str(&d.to_string()).unwrap_or_default()
     // TODO: Handle potential conversion errors more gracefully
}