// /home/inno/elights_jobes-research/backend/core-api/src/handlers/crypto.rs
use crate::db::{get_db_conn, DbPool};
use crate::error::{ApiError, internal_error};
use crate::models::{
    ApiCryptoConversionRequest, ApiCryptoConversionResponse, ApiCryptoWithdrawalRequest,
    ApiCryptoWithdrawalResponse, ApiWalletBalanceResponse
};
use crate::config::AppConfig;
use crate::middlewares::auth_guard::AuthenticatedUser;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
// Import exchange clients and traits
use cryptography_exchange::{
    CurrencyConverter, MockCurrencyConverter, RateService, MockRateService,
    BTCPayClient, // Assuming client is shared via web::Data
    #[cfg(feature = "monero_support")] MoneroWalletRpcClient, // Assuming client is shared
};
// Import domain models/utils
use domain::crypto::utils::{atomic_units_to_xmr, xmr_to_atomic_units};
use domain::models::{Wallet, Transaction, NewTransaction, TransactionType, TransactionStatus};


/// Gets a conversion quote.
pub async fn get_conversion_quote(
    _user: AuthenticatedUser,
    info: web::Json<ApiCryptoConversionRequest>,
) -> Result<impl Responder, ApiError> {
    // TODO: Use a real RateService implementation, potentially injected via web::Data
    let rate_service = MockRateService::default(); // Using mock for now

    let quote = rate_service.get_rate(&info.from_currency, &info.to_currency).await?;
    // Map quote to API response if needed, or return directly if compatible
    Ok(HttpResponse::Ok().json(quote))
}

/// Executes a crypto conversion.
pub async fn execute_crypto_conversion(
    _user: AuthenticatedUser,
    info: web::Json<ApiCryptoConversionRequest>,
    // Inject converter service if needed
) -> Result<impl Responder, ApiError> {
     // TODO: Use a real CurrencyConverter implementation via web::Data
     let converter = MockCurrencyConverter::default(); // Using mock for now

     let domain_request = cryptography_exchange::ConversionRequest {
          from_currency: info.from_currency.clone(),
          to_currency: info.to_currency.clone(),
          amount: info.amount,
     };

     let result = converter.execute_conversion(&domain_request).await?;

     // TODO: Persist conversion as a transaction in the DB

     Ok(HttpResponse::Ok().json(ApiCryptoConversionResponse{
          original_amount: result.original_amount.to_string(),
          converted_amount: result.converted_amount.to_string(),
          from_currency: result.from_currency,
          to_currency: result.to_currency,
          rate: Some(result.rate_used.to_string()),
          timestamp: chrono::Utc::now(),
     }))
}


/// Gets the balance for a specific internal crypto wallet.
pub async fn get_wallet_balance(
    db_pool: web::Data<DbPool>,
    #[cfg(feature = "monero_support")] // Conditionally inject Monero client
    monero_client: web::Data<MoneroWalletRpcClient>,
    // Inject BTCPay client if balance comes from there? Usually from node/RPC.
    // btcpay_client: web::Data<BTCPayClient>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    let wallet_id = path.into_inner();
    log::info!("User {} fetching balance for wallet ID: {}", user.username, wallet_id);

    let mut conn = get_db_conn(&db_pool)?;

    // Fetch wallet details from DB within web::block
    let wallet = web::block(move || {
        use crate::schema::wallets::dsl::*;
        use diesel::prelude::*;
         wallets
            .filter(wallet_id.eq(wallet_id))
            // TODO: Add authorization check: .filter(user_id.eq(user.user_id))
            .select(Wallet::as_select())
            .first::<Wallet>(&mut conn)
    })
    .await?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => ApiError::NotFound(format!("Wallet {} not found", wallet_id)),
        _ => internal_error(e),
    })?;

    // TODO: Ensure user is authorized to access this wallet (user.user_id == wallet.user_id)
    if wallet.user_id != user.user_id {
         return Err(ApiError::AuthorizationError("User not authorized for this wallet".to_string()));
    }

    // Get balance from appropriate source based on wallet type
    let balance: Decimal = match wallet.wallet_type.as_str() {
        "CryptoXmr" => {
            #[cfg(feature = "monero_support")]
            {
                let balance_info = monero_client.get_balance().await?;
                // Use unlocked_balance for spendable amount
                atomic_units_to_xmr(balance_info.unlocked_balance)
            }
            #[cfg(not(feature = "monero_support"))]
            {
                 log::error!("Monero feature not enabled, cannot get XMR balance");
                 return Err(ApiError::InternalError("Monero support not enabled".to_string()));
             }
        }
         "CryptoBtc" => {
             // TODO: Implement balance check via BTCPay Server or connected Bitcoin node RPC
             log::warn!("BTC balance check not implemented yet for wallet {}", wallet_id);
             Decimal::ZERO // Placeholder
         }
        _ => {
            log::error!("Balance check not supported for wallet type: {}", wallet.wallet_type);
            return Err(ApiError::BadRequest("Unsupported wallet type for balance check".to_string()));
        }
    };

    // Optionally update DB balance cache here? Or rely on external source?

    Ok(HttpResponse::Ok().json(ApiWalletBalanceResponse {
        wallet_id: wallet.wallet_id,
        currency: wallet.currency_code,
        balance: balance.to_string(),
        wallet_type: wallet.wallet_type,
        address: wallet.address,
    }))
}

/// Gets a deposit address for a given wallet type (e.g., BTC, XMR).
pub async fn get_deposit_address(
     db_pool: web::Data<DbPool>,
     #[cfg(feature = "monero_support")] monero_client: web::Data<MoneroWalletRpcClient>,
     // btcpay_client: web::Data<BTCPayClient>, // Needed for generating BTC addresses?
     user: AuthenticatedUser,
     path: web::Path<String>, // Get wallet type (e.g., "btc", "xmr")
) -> Result<impl Responder, ApiError> {
     let wallet_type_req = path.into_inner().to_uppercase();
     log::info!("User {} requesting deposit address for type: {}", user.username, wallet_type_req);

     let mut conn = get_db_conn(&db_pool)?;

     // Find user's wallet of this type OR generate a new address
     // TODO: Implement robust address generation/retrieval logic
     let address = match wallet_type_req.as_str() {
          "XMR" => {
             #[cfg(feature = "monero_support")]
             {
                 // Get primary address or generate a new subaddress?
                 // For simplicity, return primary address of account 0
                 monero_client.get_address(0).await?.address
             }
              #[cfg(not(feature = "monero_support"))]
             { return Err(ApiError::InternalError("Monero support not enabled".to_string())); }
          }
          "BTC" => {
              // TODO: Generate new address via BTCPay or Bitcoin node RPC
              "DUMMY_BTC_ADDRESS".to_string() // Placeholder
          }
          _ => return Err(ApiError::BadRequest("Unsupported crypto type for deposit address".to_string())),
     };

     // Optionally save the generated address associated with the user/wallet in DB

     Ok(HttpResponse::Ok().json(json!({ "address": address, "currency": wallet_type_req })))
 }


/// Initiates a cryptocurrency withdrawal.
pub async fn initiate_crypto_withdrawal(
    db_pool: web::Data<DbPool>,
    app_config: web::Data<Arc<AppConfig>>,
    #[cfg(feature = "monero_support")] monero_client: web::Data<MoneroWalletRpcClient>,
    // btcpay_client: web::Data<BTCPayClient>, // Needed for BTC payouts
    user: AuthenticatedUser,
    info: web::Json<ApiCryptoWithdrawalRequest>,
) -> Result<impl Responder, ApiError> {
    log::info!("User {} initiating withdrawal from wallet {}", user.username, info.source_wallet_id);

    let mut conn = get_db_conn(&db_pool)?;
    let config = app_config.get_ref().clone(); // Clone Arc<AppConfig>
    let request_info = info.into_inner(); // Move info out of Json

    // Fetch wallet, check balance, submit withdrawal (potentially blocking operations)
    let withdrawal_result = web::block(move || {
         use crate::schema::wallets::dsl as w;
         use crate::schema::transactions::dsl as t;
         use diesel::prelude::*;
         use domain::models::WalletStatus;
         use cryptography_exchange::monero_wallet::json_rpc::Destination; // Import Destination

         // --- DB Transaction ---
         conn.transaction(|conn| {
              // 1. Fetch source wallet, check owner, status, currency, and lock
              let wallet: Wallet = w::wallets
                  .filter(w::wallet_id.eq(request_info.source_wallet_id))
                  .filter(w::user_id.eq(user.user_id)) // Authorization check
                  .for_update()
                  .first(conn)
                  .map_err(|_| domain::DomainError::NotFound("Source wallet not found or not authorized".to_string()))?;

              if wallet.status != WalletStatus::Active.to_string() {
                  return Err(domain::DomainError::Validation("Source wallet is not active".to_string()));
              }
              // Infer currency from wallet if not provided in request, or validate if provided
              let currency_code = &wallet.currency_code;

              // 2. Check sufficient funds
               // Note: Balance check against RPC is better but done outside DB transaction usually.
               // Check against DB balance as initial guard.
              if wallet.balance < request_info.amount {
                  return Err(domain::DomainError::InsufficientFunds(request_info.source_wallet_id));
              }

              // 3. Create Transaction record (Pending)
              let new_tx = NewTransaction {
                  transaction_id: None,
                  debit_wallet_id: Some(request_info.source_wallet_id),
                  credit_wallet_id: None, // External destination
                  transaction_type: match currency_code.as_str() { // Set type based on currency
                        "XMR" => TransactionType::CryptoXmrSend.to_string(),
                        "BTC" => TransactionType::CryptoBtcSend.to_string(),
                        _ => return Err(domain::DomainError::NotSupported("Unsupported currency for crypto withdrawal".to_string())),
                  }.as_str(),
                  status: TransactionStatus::Pending.to_string().as_str(),
                  amount: request_info.amount,
                  currency_code: currency_code,
                  description: Some("Crypto Withdrawal"),
                  external_ref_id: None, // Set later with network Tx Hash
                  metadata: Some(json!({"destination_address": request_info.destination_address})),
              };
              let transaction: Transaction = diesel::insert_into(t::transactions)
                  .values(&new_tx)
                  .get_result(conn)?;

             // 4. Debit Source Wallet DB Balance
             let new_balance = wallet.balance - request_info.amount;
             diesel::update(w::wallets.find(request_info.source_wallet_id))
                 .set(w::balance.eq(domain::utils::decimal_to_bigdecimal(new_balance))) // Use BigDecimal
                 .execute(conn)?;

              // --- Commit DB Transaction before external call ---
              Ok((transaction, wallet.wallet_type, currency_code.to_string())) // Pass necessary info out
         }) // End DB transaction
    }).await?; // Handle blocking error

    // --- External Call (Outside DB Transaction) ---
    let (mut transaction, wallet_type, currency_code) = withdrawal_result.map_err(ApiError::DomainLogicError)?; // Get result or map domain error

    let external_call_result: Result<String, ExchangeError> = match wallet_type.as_str() {
         "CryptoXmr" => {
            #[cfg(feature = "monero_support")]
            {
                let atomic_amount = xmr_to_atomic_units(request_info.amount)
                    .ok_or(ExchangeError::InvalidInput("Invalid amount for XMR conversion".to_string()))?;
                 let dest = Destination { amount: atomic_amount, address: request_info.destination_address };
                 // Use injected Monero client
                 // Need to acquire client data again as it wasn't passed to web::block
                 // This highlights complexity of mixing shared state and web::block
                 // Alternative: Initialize client inside web::block using config? Less ideal.
                 // For now, assume we can get the client again (this needs refinement)
                 let temp_monero_client = web::Data::new(MoneroWalletRpcClient::new()?); // Re-init - **IMPROVE THIS**

                 temp_monero_client.transfer(vec![dest], request_info.payment_id, None, None).await
                     .map(|res| res.tx_hash) // Return Tx Hash on success
            }
             #[cfg(not(feature = "monero_support"))]
             { Err(ExchangeError::ConfigurationError("Monero support not enabled".to_string())) }
         }
         "CryptoBtc" => {
             // TODO: Use injected BTCPayClient to create a Payout
             // let btcpay_client = ... // Get from web::Data again (needs refinement)
             // let payout_req = CreatePayoutRequest { ... };
             // btcpay_client.create_payout(Some(&config.btcpay_default_store_id), &payout_req).await
             //   .map(|res| res.payout_id) // Return Payout ID on success
             Err(ExchangeError::InternalError("BTC payout not implemented".to_string())) // Placeholder
         }
        _ => Err(ExchangeError::UnsupportedCurrency(wallet_type)),
    };

    // --- Final DB Update (New Transaction) ---
    let mut conn = get_db_conn(&db_pool)?; // Get new connection for update
     let final_status;
     let external_ref;
     match external_call_result {
         Ok(tx_hash_or_ref) => {
             final_status = TransactionStatus::Submitted; // Or Completed if call is synchronous
             external_ref = Some(tx_hash_or_ref);
             log::info!("Crypto withdrawal successful for Tx: {}, Ref: {}", transaction.transaction_id, external_ref.as_deref().unwrap_or("-"));
         }
         Err(e) => {
             final_status = TransactionStatus::Failed;
             external_ref = None;
             log::error!("Crypto withdrawal failed for Tx: {}: {}", transaction.transaction_id, e);
              // TODO: CRITICAL - Re-credit source wallet in a new DB transaction!
         }
     };

     // Update transaction in DB (new transaction)
     transaction = web::block(move || {
          let update_tx = UpdateTransaction {
              status: Some(final_status.to_string().as_str()),
              external_ref_id: external_ref.as_deref(),
              metadata: transaction.metadata, // Keep existing metadata unless updated
              settlement_at: if final_status == TransactionStatus::Completed { Some(chrono::Utc::now()) } else { None },
          };
          diesel::update(crate::schema::transactions::table.find(transaction.transaction_id))
              .set(&update_tx)
              .get_result(&mut conn)
      }).await?.map_err(internal_error)?; // Handle potential DB error on update


    Ok(HttpResponse::Accepted().json(ApiCryptoWithdrawalResponse {
        transaction_id: transaction.transaction_id,
        status: final_status, // Return the final status after external call attempt
        message: "Withdrawal submitted".to_string(),
    }))
}


 /// Handles incoming crypto webhooks (e.g., BTCPay invoice updates).
 pub async fn handle_crypto_webhook(
     db_pool: web::Data<DbPool>,
     app_config: web::Data<Arc<AppConfig>>,
     path: web::Path<String>, // provider = btcpay | monero?
     payload: web::Bytes,
     req: HttpRequest,
 ) -> Result<impl Responder, ApiError> {
     let provider = path.into_inner();
     log::info!("Received crypto webhook from provider: {}", provider);

     // --- 1. Signature Verification ---
     match provider.as_str() {
         "btcpay" => {
             let sig_header = req.headers().get("BTCPay-Sig").and_then(|h| h.to_str().ok());
             // TODO: Get webhook secret securely from AppConfig or dedicated secrets manager
             let btcpay_webhook_secret = &app_config.btcpay_api_key; // Example: Reuse API key? Unsafe. Need dedicated secret.
             cryptography_exchange::utils::verify_btcpay_webhook_signature(
                  btcpay_webhook_secret, &payload, sig_header
             ).map_err(ApiError::from)?; // Convert ExchangeError to ApiError
         }
         // "monero" => { // Monero wallet RPC typically doesn't use webhooks, uses polling or ZMQ
         //     log::warn!("Received unexpected webhook for provider 'monero'");
         //     return Err(ApiError::BadRequest("Monero webhooks not supported".to_string()));
         // }
         _ => return Err(ApiError::BadRequest(format!("Unsupported crypto webhook provider: {}", provider))),
     }

     // --- 2. Parse Payload ---
     let event_data: serde_json::Value = serde_json::from_slice(&payload)
          .map_err(|e| ApiError::BadRequest(format!("Invalid webhook JSON payload: {}", e)))?;
     log::debug!("Crypto webhook payload parsed: {:?}", event_data);


     // --- 3. Process Event & Update DB ---
     let mut conn = get_db_conn(&db_pool)?;
     match provider.as_str() {
          "btcpay" => {
              let event: WebhookInvoiceEvent = serde_json::from_value(event_data)
                  .map_err(|e| ApiError::BadRequest(format!("Failed to parse BTCPay webhook event: {}", e)))?;

             // Find internal transaction using event.invoice_id (likely stored in external_ref_id or metadata)
             // TODO: Implement logic to find transaction by invoice_id
             // let internal_tx_id = find_tx_by_btcpay_invoice_id(&mut conn, &event.invoice_id).await?;

             // Map BTCPay status (event.r#type) to internal TransactionStatus
             let new_status = match event.r#type.as_str() {
                 "InvoiceSettled" => TransactionStatus::Completed, // Or Settled
                 "InvoiceProcessing" => TransactionStatus::Processing,
                  "InvoiceInvalid" | "InvoiceExpired" => TransactionStatus::Failed,
                 // Handle other events like InvoiceReceivedPayment if needed
                 _ => { log::info!("Ignoring BTCPay event type: {}", event.r#type); return Ok(HttpResponse::Ok().finish()); }
             };

             // TODO: Update transaction status using PaymentProcessor or directly
             // processor.update_payment_status(internal_tx_id, new_status, Some(&event.invoice_id), ...).await?;
              log::info!("Processed BTCPay webhook for Invoice: {}, Type: {}", event.invoice_id, event.r#type);
          }
          _ => unreachable!(), // Already handled provider check
     }

     Ok(HttpResponse::Ok().finish()) // Return 200 OK
 }


 // --- Helper Error Conversion ---
 impl From<cryptography_exchange::ExchangeError> for ApiError {
     fn from(err: cryptography_exchange::ExchangeError) -> Self {
         log::warn!("ExchangeError occurred: {}", err); // Log the specific exchange error
         match err {
             cryptography_exchange::ExchangeError::ConfigurationError(s) => ApiError::ConfigurationError(s),
             cryptography_exchange::ExchangeError::RequestError(e) => ApiError::ExternalServiceError(format!("HTTP Client Error: {}", e)),
             cryptography_exchange::ExchangeError::NotInitialized(s) => ApiError::InternalError(format!("Client not initialized: {}", s)),
             cryptography_exchange::ExchangeError::ApiError { status, body } => ApiError::ExternalServiceError(format!("External API Error {}: {}", status, body)),
             cryptography_exchange::ExchangeError::JsonRpcError { code, message } => ApiError::ExternalServiceError(format!("RPC Error {}: {}", code, message)),
             cryptography_exchange::ExchangeError::ResponseParseError(s) => ApiError::ExternalServiceError(format!("Failed to parse external response: {}", s)),
             cryptography_exchange::ExchangeError::TimeoutError => ApiError::TimeoutError("External service timeout".to_string()),
             cryptography_exchange::ExchangeError::InvalidInput(s) => ApiError::BadRequest(s), // Treat invalid input as Bad Request
             cryptography_exchange::ExchangeError::CryptoOperationFailed(s) => ApiError::InternalError(format!("Crypto Operation Error: {}", s)),
             cryptography_exchange::ExchangeError::ConversionError(s) => ApiError::InternalError(format!("Conversion Error: {}", s)),
             cryptography_exchange::ExchangeError::UnsupportedCurrency(s) => ApiError::BadRequest(format!("Unsupported currency: {}", s)),
             cryptography_exchange::ExchangeError::WebhookVerificationError(s) => ApiError::AuthenticationError(format!("Webhook Auth Failed: {}", s)), // Treat as Auth error
             cryptography_exchange::ExchangeError::RateFetchingError(s) => ApiError::ExternalServiceError(format!("Rate Fetching Error: {}", s)),
             cryptography_exchange::ExchangeError::InternalError(s) => ApiError::InternalError(s), // Pass through internal errors
         }
     }
 }
use cryptography_exchange::models::WebhookInvoiceEvent;
use domain::models::TransactionStatus; // Import domain status enum
use std::str::FromStr;