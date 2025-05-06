// /home/inno/elights_jobes-research/backend/domain/src/payments/card.rs
use diesel::prelude::*;
use crate::models::{Transaction, NewTransaction, Wallet, TransactionType, TransactionStatus, CardDetails as TxCardDetails, UpdateTransaction};
use crate::error::DomainError;
use crate::payments::validator::{validate_card_details, ValidationContext};
use crate::payments::gateway::{PaymentGateway, PaymentGatewayRequest, PaymentGatewayResponse, PaymentIntent, PaymentMethodDetails}; // Use gateway trait
use rust_decimal::Decimal;
use uuid::Uuid;

// Note: Live card processing requires strict adherence to PCI DSS compliance standards.
// Sensitive card data (full PAN, CVV) should NOT be stored in your database.
// Typically, you only store the last 4 digits, expiry, card type, and a token from the payment gateway.

/// Processes a card payment authorization.
/// This contacts the payment gateway to authorize the amount on the cardholder's account.
pub async fn process_card_authorization(
    conn: &mut PgConnection,
    gateway: &dyn PaymentGateway, // Inject gateway implementation
    initiating_user_id: Uuid,
    wallet_to_charge_token: &str, // Gateway's token representing the card (NOT the raw card number)
    amount: Decimal,
    currency: &str, // ISO 4217
    description: &str,
    metadata: Option<serde_json::Value>,
) -> Result<Transaction, DomainError> {
    log::info!("Processing Card Authorization for amount {} {}", amount, currency);

    // 1. Create initial transaction record (Status: Pending/RequiresAction)
    let new_tx = NewTransaction {
        transaction_id: None,
        debit_wallet_id: None, // External cardholder
        credit_wallet_id: None, // Funds not yet moved
        transaction_type: TransactionType::CardAuthorization.to_string().as_str(),
        status: TransactionStatus::Pending.to_string().as_str(),
        amount,
        currency_code: currency,
        description: Some(description),
        external_ref_id: None, // Gateway reference will be added later
        metadata: metadata.clone(),
    };
    let mut transaction: Transaction = diesel::insert_into(crate::schema::transactions::table)
        .values(&new_tx)
        .get_result(conn)?;

    // 2. Prepare request for the payment gateway
    let request = PaymentGatewayRequest {
        amount,
        currency: currency.to_string(),
        payment_method: PaymentMethodDetails::CardToken(wallet_to_charge_token.to_string()),
        intent: PaymentIntent::Authorize,
        description: Some(description.to_string()),
        customer_id: Some(initiating_user_id.to_string()), // Example customer ref
        metadata: metadata.clone(),
    };

    // 3. Call the payment gateway
    let gateway_response = gateway.submit_payment(request).await?; // Use injected gateway

    // 4. Update transaction based on gateway response
    let final_status;
    let external_ref = Some(gateway_response.gateway_transaction_id.as_str());
    let mut updated_metadata = transaction.metadata.clone().unwrap_or_else(|| serde_json::json!({}));
    if let Some(details) = gateway_response.details {
        // Store relevant non-sensitive details from gateway if needed
        if let serde_json::Value::Object(mut map) = updated_metadata {
            map.insert("gateway_details".to_string(), details);
            updated_metadata = serde_json::Value::Object(map);
        }
    }


    if gateway_response.success {
        log::info!("Card authorization successful. Gateway Ref: {}", gateway_response.gateway_transaction_id);
        final_status = TransactionStatus::Authorized;
        // Optionally store authorization code in metadata
         if let serde_json::Value::Object(ref mut map) = updated_metadata {
            // Example: Storing auth code if returned by gateway
            // map.insert("authorization_code".to_string(), json!(gateway_response.auth_code));
        }
    } else {
        log::error!("Card authorization failed. Gateway Ref: {}, Reason: {:?}",
            gateway_response.gateway_transaction_id, gateway_response.error_message);
        final_status = TransactionStatus::Failed;
        // TODO: Update metadata with failure reason/code from gateway
        // Store error message in transaction record? Or just log?
    }

    let update_tx = UpdateTransaction {
        status: Some(final_status.to_string().as_str()),
        external_ref_id: external_ref,
        metadata: Some(updated_metadata),
        settlement_at: None,
    };
    transaction = diesel::update(crate::schema::transactions::table.find(transaction.transaction_id))
        .set(&update_tx)
        .get_result(conn)?;

    if final_status == TransactionStatus::Failed {
        // Return specific error if auth failed
        return Err(DomainError::CardProcessing(
            gateway_response.error_message.unwrap_or_else(|| "Authorization failed".to_string())
        ));
    }

    Ok(transaction)
}


/// Processes a card payment capture.
/// Captures funds previously authorized.
pub async fn process_card_capture(
    conn: &mut PgConnection,
    gateway: &dyn PaymentGateway, // Inject gateway implementation
    authorization_transaction_id: Uuid, // ID of the original authorization transaction
    capture_amount: Option<Decimal>, // Optional: Capture less than authorized amount
) -> Result<Transaction, DomainError> {
    log::info!("Processing Card Capture for Auth Tx ID: {}", authorization_transaction_id);

    // 1. Find the original authorization transaction
    let auth_tx: Transaction = crate::schema::transactions::table
        .find(authorization_transaction_id)
        .first(conn)
        .map_err(|_| DomainError::NotFound(format!("Authorization transaction {} not found", authorization_transaction_id)))?;

    // 2. Validate status and amount
    if auth_tx.status != TransactionStatus::Authorized.to_string() {
        return Err(DomainError::Validation("Cannot capture transaction that is not Authorized".to_string()));
    }
    let amount_to_capture = capture_amount.unwrap_or(auth_tx.amount);
    if amount_to_capture <= Decimal::ZERO || amount_to_capture > auth_tx.amount {
        return Err(DomainError::Validation("Invalid capture amount".to_string()));
    }
    let gateway_ref = auth_tx.external_ref_id.as_deref()
        .ok_or_else(|| DomainError::Validation("Missing gateway reference on authorization transaction".to_string()))?;

    // 3. Create Capture Transaction record (linked to Auth) - Optional, depends on model
    // Or update the existing auth transaction status to Processing/Completed directly.
    // For simplicity, let's update the existing one here. Mark as Processing first.
    let mut transaction = diesel::update(crate::schema::transactions::table.find(auth_tx.transaction_id))
         .set(crate::schema::transactions::status.eq(TransactionStatus::Processing.to_string()))
         .get_result::<Transaction>(conn)?;


    // 4. Prepare gateway request
    let request = PaymentGatewayRequest {
        amount: amount_to_capture,
        currency: auth_tx.currency_code.clone(),
        payment_method: PaymentMethodDetails::GatewayReference(gateway_ref.to_string()),
        intent: PaymentIntent::Capture,
        description: Some(format!("Capture for Auth {}", authorization_transaction_id)),
        customer_id: None, // Usually not needed for capture
        metadata: None, // Or pass specific capture metadata
    };

    // 5. Call the payment gateway
    let gateway_response = gateway.submit_payment(request).await?;

    // 6. Update transaction status based on response
    let final_status;
     let mut updated_metadata = transaction.metadata.clone().unwrap_or_else(|| serde_json::json!({}));
     if let Some(details) = gateway_response.details {
         if let serde_json::Value::Object(mut map) = updated_metadata {
             map.insert("gateway_capture_details".to_string(), details);
             updated_metadata = serde_json::Value::Object(map);
         }
     }

    if gateway_response.success {
         log::info!("Card capture successful. Gateway Ref: {}", gateway_response.gateway_transaction_id);
         final_status = TransactionStatus::Completed; // Assume capture means funds moved
         // TODO: Credit the appropriate internal wallet based on the original transaction context
         // update_wallet_balance(conn, credit_wallet_id, amount_to_capture)?;
    } else {
         log::error!("Card capture failed. Gateway Ref: {}, Reason: {:?}",
             gateway_response.gateway_transaction_id, gateway_response.error_message);
         final_status = TransactionStatus::Failed;
         // Optionally revert status back from Processing if capture fails? Or keep as Failed.
    }

    let update_tx = UpdateTransaction {
        status: Some(final_status.to_string().as_str()),
        external_ref_id: Some(gateway_response.gateway_transaction_id.as_str()), // Capture might have new ref
        metadata: Some(updated_metadata),
        settlement_at: if final_status == TransactionStatus::Completed { Some(Utc::now()) } else { None }, // Approximate settlement
    };
    transaction = diesel::update(crate::schema::transactions::table.find(transaction.transaction_id))
        .set(&update_tx)
        .get_result(conn)?;

     if final_status == TransactionStatus::Failed {
        return Err(DomainError::CardProcessing(
            gateway_response.error_message.unwrap_or_else(|| "Capture failed".to_string())
        ));
    }

    Ok(transaction)
}

/// Processes a card payment refund.
/// Refunds a previously captured amount.
pub async fn process_card_refund(
    conn: &mut PgConnection,
    gateway: &dyn PaymentGateway, // Inject gateway implementation
    capture_transaction_id: Uuid, // ID of the original capture transaction
    refund_amount: Option<Decimal>, // Optional: Refund less than captured amount
    reason: Option<&str>,
) -> Result<Transaction, DomainError> {
     log::info!("Processing Card Refund for Capture Tx ID: {}", capture_transaction_id);

    // 1. Find the original capture transaction
    let capture_tx: Transaction = crate::schema::transactions::table
        .find(capture_transaction_id)
        .first(conn)
        .map_err(|_| DomainError::NotFound(format!("Capture transaction {} not found", capture_transaction_id)))?;

    // 2. Validate status and amount
     if capture_tx.status != TransactionStatus::Completed.to_string() && capture_tx.status != TransactionStatus::Settled.to_string() {
        return Err(DomainError::Validation("Cannot refund transaction that is not Completed/Settled".to_string()));
    }
    let amount_to_refund = refund_amount.unwrap_or(capture_tx.amount);
    if amount_to_refund <= Decimal::ZERO || amount_to_refund > capture_tx.amount {
        return Err(DomainError::Validation("Invalid refund amount".to_string()));
    }
     let gateway_ref = capture_tx.external_ref_id.as_deref()
        .ok_or_else(|| DomainError::Validation("Missing gateway reference on capture transaction".to_string()))?;

    // 3. Create Refund Transaction record (linked to Capture)
    let new_tx = NewTransaction {
        transaction_id: None,
        debit_wallet_id: capture_tx.credit_wallet_id, // Debit the internal wallet credited previously
        credit_wallet_id: None, // External cardholder
        transaction_type: TransactionType::CardRefund.to_string().as_str(),
        status: TransactionStatus::Pending.to_string().as_str(),
        amount: amount_to_refund,
        currency_code: &capture_tx.currency_code,
        description: Some(reason.unwrap_or("Card Refund")),
        external_ref_id: None, // Will get gateway ref for refund
        metadata: Some(serde_json::json!({ "original_transaction_id": capture_transaction_id })),
    };
    let mut transaction: Transaction = diesel::insert_into(crate::schema::transactions::table)
        .values(&new_tx)
        .get_result(conn)?;

    // TODO: Implement Debit from internal wallet (part of DB transaction)
    // update_wallet_balance(conn, capture_tx.credit_wallet_id.unwrap(), -amount_to_refund)?;

    // 4. Prepare gateway request
     let request = PaymentGatewayRequest {
        amount: amount_to_refund,
        currency: capture_tx.currency_code.clone(),
        payment_method: PaymentMethodDetails::GatewayReference(gateway_ref.to_string()), // Reference the original txn
        intent: PaymentIntent::Refund,
        description: Some(reason.unwrap_or("Refund").to_string()),
        customer_id: None,
        metadata: None,
    };

    // 5. Call the payment gateway
    let gateway_response = gateway.submit_payment(request).await?;

    // 6. Update refund transaction status
    let final_status;
    let mut updated_metadata = transaction.metadata.clone().unwrap_or_else(|| serde_json::json!({}));
     if let Some(details) = gateway_response.details {
         if let serde_json::Value::Object(mut map) = updated_metadata {
             map.insert("gateway_refund_details".to_string(), details);
             updated_metadata = serde_json::Value::Object(map);
         }
     }

    if gateway_response.success {
         log::info!("Card refund successful. Gateway Ref: {}", gateway_response.gateway_transaction_id);
         final_status = TransactionStatus::Completed;
    } else {
         log::error!("Card refund failed. Gateway Ref: {}, Reason: {:?}",
             gateway_response.gateway_transaction_id, gateway_response.error_message);
         final_status = TransactionStatus::Failed;
         // TODO: Handle refund failure - Re-credit internal wallet?
         // update_wallet_balance(conn, capture_tx.credit_wallet_id.unwrap(), amount_to_refund)?; // Reversal
    }

     let update_tx = UpdateTransaction {
        status: Some(final_status.to_string().as_str()),
        external_ref_id: Some(gateway_response.gateway_transaction_id.as_str()),
        metadata: Some(updated_metadata),
        settlement_at: if final_status == TransactionStatus::Completed { Some(Utc::now()) } else { None }, // Approximate settlement
    };
    transaction = diesel::update(crate::schema::transactions::table.find(transaction.transaction_id))
        .set(&update_tx)
        .get_result(conn)?;

     if final_status == TransactionStatus::Failed {
        return Err(DomainError::CardProcessing(
            gateway_response.error_message.unwrap_or_else(|| "Refund failed".to_string())
        ));
    }

    Ok(transaction)
}