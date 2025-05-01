// /home/inno/elights_jobes-research/backend/core-api/src/handlers/payments.rs
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::error::ApiError;
use domain::payments::{
    process_ach_payment, process_card_payment, process_check_payment, process_wire_payment, // Example imports
};
use domain::models::{TransactionType, TransactionStatus}; // Use domain enums
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct InitiatePaymentRequest {
    amount: Decimal, // Use Decimal for accuracy
    currency: String, // e.g., "USD", "EUR"
    payment_type: TransactionType, // Use enum from domain
    // Add specific details based on payment_type
    source_account_id: Option<i32>, // Internal account ID
    beneficiary_account_id: Option<i32>, // Internal account ID
    beneficiary_details: Option<BeneficiaryDetails>, // External details
    card_details: Option<CardDetails>, // For card payments
    ach_details: Option<AchDetails>, // For ACH
    wire_details: Option<WireDetails>, // For Wire
    check_details: Option<CheckDetails>, // For Check
    description: Option<String>,
    metadata: Option<serde_json::Value>, // Allow arbitrary metadata
}

#[derive(Debug, Deserialize)]
pub struct BeneficiaryDetails {
    name: String,
    account_number: String, // Could be IBAN etc.
    bank_bic_swift: Option<String>,
    bank_name: Option<String>,
    // Add address etc. if needed
}

#[derive(Debug, Deserialize)]
pub struct CardDetails {
    card_number: String,
    expiry_month: u8,
    expiry_year: u16,
    cvv: String,
}

#[derive(Debug, Deserialize)]
pub struct AchDetails {
    routing_number: String,
    account_number: String,
}

#[derive(Debug, Deserialize)]
pub struct WireDetails {
    swift_bic: String,
    account_number: String, // IBAN or other format
    beneficiary_name: String,
    // Add intermediary bank info, purpose codes etc.
}

#[derive(Debug, Deserialize)]
pub struct CheckDetails {
     payee_name: String,
     routing_number: String,
     account_number: String,
     check_number: Option<String>,
}


#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    transaction_id: Uuid, // Use UUID
    status: TransactionStatus, // Use enum
    message: String,
}

/// Initiates a payment based on the request type.
pub async fn initiate_payment(
    db_pool: web::Data<PgPool>,
    info: web::Json<InitiatePaymentRequest>,
) -> Result<impl Responder, ApiError> {
    log::info!("Initiating payment type: {:?}", info.payment_type);

    // TODO: Implement authorization check (which user is initiating?)

    // --- Basic Validation ---
    if info.amount <= Decimal::ZERO {
        return Err(ApiError::ValidationError("Payment amount must be positive".to_string()));
    }
    // Add currency validation

    // --- Placeholder Logic ---
    // 1. Create Transaction record in DB with PENDING status
    let transaction_id = Uuid::new_v4();
    // TODO: Insert transaction into database `core_schema.transactions`
    // let initial_status = TransactionStatus::Pending;
    // let db_result = sqlx::query!(...) -> map_err(|e| ApiError::DatabaseError(...))?;

    // 2. Route to appropriate domain service based on payment_type
    let processing_result: Result<String, domain::DomainError> = match info.payment_type {
        TransactionType::Ach => {
            let details = info.ach_details.as_ref().ok_or_else(|| ApiError::BadRequest("ACH details missing".to_string()))?;
            process_ach_payment(info.amount, &details.routing_number, &details.account_number)
        }
        TransactionType::Wire => {
             let details = info.wire_details.as_ref().ok_or_else(|| ApiError::BadRequest("Wire details missing".to_string()))?;
             process_wire_payment(info.amount, &info.currency, &details.swift_bic, &details.account_number, &details.beneficiary_name)
         }
        TransactionType::Card => {
             let details = info.card_details.as_ref().ok_or_else(|| ApiError::BadRequest("Card details missing".to_string()))?;
             process_card_payment(&details.card_number, details.expiry_month, details.expiry_year, &details.cvv, info.amount)
         }
         TransactionType::Check => {
             let details = info.check_details.as_ref().ok_or_else(|| ApiError::BadRequest("Check details missing".to_string()))?;
             process_check_payment(&details.payee_name, &details.routing_number, &details.account_number, details.check_number.as_deref(), info.amount)
         }
        // Handle other types (InternalTransfer, Crypto)
        _ => Err(domain::DomainError::Validation(format!("Unsupported payment type: {:?}", info.payment_type))),
    };

    // 3. Update Transaction status in DB based on processing result
    let final_status = match processing_result {
        Ok(processor_ref) => {
            log::info!("Payment {} initiated successfully via processor. Ref: {}", transaction_id, processor_ref);
             // TODO: Update transaction status to PROCESSING or COMPLETED in DB
             TransactionStatus::Processing // Or Completed if synchronous
        }
        Err(e) => {
            log::error!("Payment initiation {} failed: {}", transaction_id, e);
             // TODO: Update transaction status to FAILED in DB
             return Err(ApiError::DomainLogicError(e)); // Return error to client
        }
    };

    Ok(HttpResponse::Accepted().json(PaymentResponse { // Use Accepted (202) for async processing
        transaction_id,
        status: final_status,
        message: format!("Payment {:?} initiated.", info.payment_type),
    }))
    // --- End Placeholder Logic ---
}

/// Gets the status of a specific payment transaction.
pub async fn get_payment_status(
    db_pool: web::Data<PgPool>,
    path: web::Path<Uuid>, // Get transaction ID from path
) -> Result<impl Responder, ApiError> {
    let transaction_id = path.into_inner();
    log::info!("Fetching status for transaction ID: {}", transaction_id);

    // --- Placeholder Logic ---
    // 1. Fetch transaction from database by ID
    // Example using SQLx:
    // let transaction_result = sqlx::query_as!(
    //     domain::models::Transaction, // Use actual struct compatible with FromRow
    //     "SELECT ... FROM core_schema.transactions WHERE id = $1",
    //     transaction_id
    // )
    // .fetch_optional(db_pool.get_ref())
    // .await;
    //
    // let transaction = match transaction_result {
    //     Ok(Some(tx)) => tx,
    //     Ok(None) => return Err(ApiError::NotFoundError(format!("Transaction {} not found", transaction_id))),
    //     Err(e) => {
    //         log::error!("Database error fetching transaction {}: {}", transaction_id, e);
    //         return Err(ApiError::DatabaseError("Failed to fetch transaction status".to_string()));
    //     }
    // };

    // Dummy response
    let dummy_status = TransactionStatus::Completed; // Replace with actual status from DB

    Ok(HttpResponse::Ok().json(PaymentResponse {
        transaction_id,
        status: dummy_status,
        message: "Status retrieved".to_string(),
    }))
    // --- End Placeholder Logic ---
}

/// Handles incoming payment webhooks (e.g., from Stripe, ACH returns).
pub async fn handle_payment_webhook(
    db_pool: web::Data<PgPool>,
    payload: web::Bytes, // Process raw bytes to verify signature before parsing JSON
    req: actix_web::HttpRequest,
) -> Result<impl Responder, ApiError> {
    log::info!("Received payment webhook");

    // --- Placeholder Logic ---
    // 1. Verify webhook signature (essential for security!)
    //    - Get signature from request headers (e.g., 'Stripe-Signature')
    //    - Get webhook secret from config/env
    //    - Use the provider's library to verify the signature against the raw payload bytes.
    //    - If verification fails, return Unauthorized or BadRequest.
    // let signature = req.headers().get("Webhook-Signature").map(|h| h.to_str().unwrap_or(""));
    // if !verify_webhook_signature(&payload, signature, "webhook_secret") {
    //      return Err(ApiError::AuthenticationError("Invalid webhook signature".to_string()));
    // }

    // 2. Parse the payload (now that signature is verified)
    let event_data: serde_json::Value = serde_json::from_slice(&payload)
         .map_err(|e| ApiError::BadRequest(format!("Invalid webhook JSON payload: {}", e)))?;

    // 3. Process the event data
    //    - Identify event type (e.g., 'charge.succeeded', 'ach.returned')
    //    - Extract relevant transaction identifiers
    //    - Update the corresponding transaction status in the database
    log::info!("Processing webhook event: {:?}", event_data.get("type"));
    // TODO: Implement event processing and DB update logic

    Ok(HttpResponse::Ok().finish()) // Respond with 200 OK to acknowledge receipt
     // --- End Placeholder Logic ---
}