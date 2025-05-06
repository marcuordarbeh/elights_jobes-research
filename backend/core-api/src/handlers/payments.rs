// /home/inno/elights_jobes-research/backend/core-api/src/handlers/payments.rs
use crate::db::{get_db_conn, DbPool};
use crate::error::{ApiError, internal_error};
use crate::models::{ApiInitiatePaymentRequest, ApiPaymentResponse, ApiPaymentStatusResponse};
use crate::config::AppConfig;
use crate::middlewares::auth_guard::AuthenticatedUser; // Import claims from auth middleware
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use domain::payments::{PaymentProcessor, PaymentRequest as DomainPaymentRequest}; // Use domain processor/request
use domain::models::{Transaction, AchDetails, WireDetails, CheckDetails}; // Import domain details
use std::sync::Arc;
use uuid::Uuid;
use bank_integrations::gateway::MockPaymentGateway; // Using mock gateway for now

/// Initiates a payment. Requires authentication.
pub async fn initiate_payment(
    db_pool: web::Data<DbPool>,
    _app_config: web::Data<Arc<AppConfig>>, // Get config if needed by processor
    user: AuthenticatedUser, // Claims from AuthGuard middleware
    info: web::Json<ApiInitiatePaymentRequest>,
) -> Result<impl Responder, ApiError> {
    log::info!("User {} initiating payment: Type={:?}, Amount={} {}",
        user.username, info.payment_type, info.amount, info.currency);

    let mut conn = get_db_conn(&db_pool)?;

    // --- Create Domain Payment Request ---
    // TODO: Map API request fields to DomainPaymentRequest fields carefully
    // Requires parsing API details into domain detail structs
    let domain_ach_details: Option<AchDetails> = if let (Some(r), Some(a)) = (&info.ach_routing, &info.ach_account) {
         Some(AchDetails { routing_number: r.clone(), account_number: a.clone() })
    } else { None };
    // TODO: Map other detail types (Wire, Check) similarly

    let domain_request = DomainPaymentRequest {
        initiating_user_id: user.user_id, // Get user ID from JWT claims
        amount: info.amount,
        currency: &info.currency,
        payment_type: info.payment_type.clone(),
        source_wallet_id: info.source_wallet_id,
        destination_wallet_id: info.destination_wallet_id,
        ach_details: domain_ach_details.as_ref(), // Pass references
        wire_details: None, // TODO: Map from info
        card_token: info.card_token.as_deref(),
        check_details: None, // TODO: Map from info
        crypto_address: None, // Not for fiat payments
        description: info.description.as_deref().unwrap_or("Payment Initiation"),
        metadata: info.metadata.clone(),
    };

    // --- Use Payment Processor ---
    // TODO: Inject real card gateway implementation based on config
    let mock_card_gateway = MockPaymentGateway::default();

    let processor = PaymentProcessor::new(&mut conn, &mock_card_gateway);

    // Processor handles DB transaction, validation, debit, external calls (stubs), status updates
    // Run the processor logic in a blocking thread if it makes synchronous DB calls heavily
    let transaction_result = web::block(move || processor.process_outbound_payment(domain_request))
        .await? // Handle blocking error
        .map_err(ApiError::DomainLogicError)?; // Map DomainError

    Ok(HttpResponse::Accepted().json(ApiPaymentResponse {
        transaction_id: transaction_result.transaction_id,
        status: domain::models::TransactionStatus::from_str(&transaction_result.status)
            .unwrap_or(domain::models::TransactionStatus::Unknown), // Convert string back to enum
        message: format!("Payment {:?} submitted successfully.", info.payment_type),
        created_at: transaction_result.created_at,
    }))
}

/// Gets the status of a specific payment transaction. Requires authentication.
pub async fn get_payment_status(
    db_pool: web::Data<DbPool>,
    _user: AuthenticatedUser, // Ensure user is authenticated
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    let transaction_id = path.into_inner();
    log::info!("Fetching status for transaction ID: {}", transaction_id);

    let mut conn = get_db_conn(&db_pool)?;

    // Fetch transaction directly using Diesel within web::block
    let transaction = web::block(move || {
        use crate::schema::transactions::dsl::*;
        use diesel::prelude::*;
        transactions
            .find(transaction_id)
            .select(Transaction::as_select()) // Select full domain model
            .first::<Transaction>(&mut conn)
    })
    .await? // Handle blocking error
    .map_err(|e| match e { // Map Diesel error
         diesel::result::Error::NotFound => ApiError::NotFound(format!("Transaction {} not found", transaction_id)),
         _ => internal_error(e),
    })?;

    // TODO: Add authorization check - does the authenticated user own this transaction?

    // Map domain::Transaction to ApiPaymentStatusResponse
    let response = ApiPaymentStatusResponse {
        transaction_id: transaction.transaction_id,
        status: domain::models::TransactionStatus::from_str(&transaction.status).unwrap_or(domain::models::TransactionStatus::Unknown),
        transaction_type: domain::models::TransactionType::from_str(&transaction.transaction_type).unwrap_or(domain::models::TransactionType::Unknown),
        amount: transaction.amount.to_string(),
        currency: transaction.currency_code,
        description: transaction.description,
        created_at: transaction.created_at,
        updated_at: transaction.updated_at,
        settlement_at: transaction.settlement_at,
        external_ref_id: transaction.external_ref_id,
        metadata: transaction.metadata,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Handles incoming payment webhooks. Public endpoint, requires signature verification.
pub async fn handle_payment_webhook(
    db_pool: web::Data<DbPool>,
    _app_config: web::Data<Arc<AppConfig>>, // For webhook secrets
    path: web::Path<String>, // Get provider name from path
    payload: web::Bytes, // Raw payload for signature verification
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let provider = path.into_inner();
    log::info!("Received payment webhook from provider: {}", provider);

    // --- 1. Signature Verification ---
    // TODO: Implement signature verification based on provider
    match provider.as_str() {
        // "stripe" => {
        //     let sig_header = req.headers().get("Stripe-Signature")....;
        //     let secret = app_config.stripe_webhook_secret;
        //     verify_stripe_signature(&payload, sig_header, &secret)?;
        // }
        "mock" => { // Allow mock provider for testing
            log::warn!("Processing mock webhook - skipping signature verification.");
         }
        _ => {
            log::error!("Unsupported webhook provider: {}", provider);
            return Err(ApiError::BadRequest("Unsupported webhook provider".to_string()));
        }
    }

    // --- 2. Parse Payload ---
     let event_data: serde_json::Value = serde_json::from_slice(&payload)
         .map_err(|e| ApiError::BadRequest(format!("Invalid webhook JSON payload: {}", e)))?;
    log::debug!("Webhook payload parsed: {:?}", event_data);


    // --- 3. Process Event & Update DB ---
    // Use PaymentProcessor or dedicated webhook handler service
    let mut conn = get_db_conn(&db_pool)?;
    // TODO: Implement processor.handle_webhook_event(...)
    // This function needs to parse the provider-specific payload,
    // map it to internal status updates (e.g., Completed, Failed, Returned),
    // and call processor.update_payment_status(...) within a DB transaction.
    // Example conceptual call:
    // processor.handle_webhook_event(&provider, event_data).await?;

    log::info!("Webhook from provider '{}' processed successfully.", provider);
    Ok(HttpResponse::Ok().finish()) // Return 200 OK to acknowledge
}

// Helper for enum FromStr (add to domain models or utils)
impl std::str::FromStr for domain::models::TransactionStatus {
     type Err = ();
     fn from_str(s: &str) -> Result<Self, Self::Err> {
         match s {
             "Pending" => Ok(Self::Pending), "Processing" => Ok(Self::Processing),
             "RequiresAction" => Ok(Self::RequiresAction), "Authorized" => Ok(Self::Authorized),
             "Submitted" => Ok(Self::Submitted), "Settled" => Ok(Self::Settled),
             "Completed" => Ok(Self::Completed), "Failed" => Ok(Self::Failed),
             "Cancelled" => Ok(Self::Cancelled), "Returned" => Ok(Self::Returned),
             "Chargeback" => Ok(Self::Chargeback), "Expired" => Ok(Self::Expired),
             _ => Ok(Self::Unknown), // Default or Err(())
         }
     }
 }
  impl std::str::FromStr for domain::models::TransactionType {
       type Err = ();
       fn from_str(s: &str) -> Result<Self, Self::Err> {
           // Match strings based on enum variants defined in domain::models
           match s {
                "AchCredit" => Ok(Self::AchCredit), "AchDebit" => Ok(Self::AchDebit),
                "WireOutbound" => Ok(Self::WireOutbound), "WireInbound" => Ok(Self::WireInbound),
                // ... add all other variants ...
                _ => Ok(Self::Unknown) // Default or Err(())
           }
       }
   }