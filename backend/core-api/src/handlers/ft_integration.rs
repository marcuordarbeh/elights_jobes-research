// /home/inno/elights_jobes-research/backend/core-api/src/handlers/ft_integration.rs
use crate::db::{get_db_conn, DbPool};
use crate::error::{ApiError, internal_error};
use crate::models::FtNotificationPayload; // API model for incoming notification
use crate::services::ft_client::FtApiClient; // FT API client service
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

/// Handles incoming push notifications from the FT API.
pub async fn handle_ft_notification(
    db_pool: web::Data<DbPool>,
    ft_client: web::Data<FtApiClient>, // Inject FT Client
    payload: web::Json<FtNotificationPayload>, // Parsed JSON payload
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    log::info!("Received FT push notification");
    log::debug!("FT Notification Payload: {:?}", payload);

    // --- 1. Signature/Source Verification (Recommended but FT docs might not specify standard) ---
    // TODO: Implement verification if FT provides a mechanism (e.g., shared secret HMAC, IP check)
    // This is crucial to ensure notifications are genuinely from FT.
    // verify_ft_notification_source(&req)?;

    // --- 2. Process Notifications ---
    let notifications = payload.into_inner().notifications;
    if notifications.is_empty() {
        log::info!("FT notification payload contained no items.");
        return Ok(HttpResponse::Ok().finish()); // Acknowledge receipt
    }

    // Process each notification item asynchronously or queue for background processing
    // For simplicity, process sequentially here. Use tokio::spawn for concurrent fetching.
    for item in notifications {
         log::info!("Processing FT notification item. Type: {}, URL: {}", item.notification_type, item.api_url);
         // Extract content UUID from the api_url
         if let Some(content_uuid) = extract_uuid_from_url(&item.api_url) {
              log::info!("Extracted content UUID: {}", content_uuid);
             // Fetch the updated content using the FtApiClient
             match ft_client.get_content(&content_uuid.to_string()).await {
                 Ok(content_data) => {
                      // TODO: Store/Update the fetched content data in the local database
                      // store_ft_content_in_db(&db_pool, content_uuid, &content_data).await?;
                      log::info!("Successfully fetched and processed content {}", content_uuid);
                 }
                 Err(e) => {
                      log::error!("Failed to fetch FT content {} from notification: {}", content_uuid, e);
                      // Decide how to handle errors (retry? log? ignore?)
                 }
             }
         } else {
              log::warn!("Could not extract UUID from FT notification URL: {}", item.api_url);
         }
    }

    Ok(HttpResponse::Ok().finish()) // Return 200 OK to acknowledge receipt
}

/// Example handler to fetch specific FT content (e.g., for debugging or direct access).
pub async fn get_ft_content(
    ft_client: web::Data<FtApiClient>,
    path: web::Path<String>, // Content UUID from path
) -> Result<impl Responder, ApiError> {
    let content_uuid = path.into_inner();
    log::info!("Fetching FT content with UUID: {}", content_uuid);

    // Basic UUID validation
    if Uuid::parse_str(&content_uuid).is_err() {
        return Err(ApiError::BadRequest("Invalid content UUID format".to_string()));
    }

    let content_data = ft_client.get_content(&content_uuid).await?;

    // Return the raw JSON content received from FT API
    Ok(HttpResponse::Ok().json(content_data))
}


// --- Helper Functions ---

/// Extracts UUID from common FT API URL patterns. Basic implementation.
fn extract_uuid_from_url(url: &str) -> Option<Uuid> {
    // Example patterns:
    // http://api.ft.com/content/uuid-goes-here
    // http://api.ft.com/things/uuid-goes-here
    url.split('/')
        .last()
        .and_then(|potential_uuid| Uuid::parse_str(potential_uuid).ok())
}

// Placeholder for webhook source verification
// fn verify_ft_notification_source(req: &HttpRequest) -> Result<(), ApiError> {
//     // TODO: Implement verification logic based on FT documentation (if available)
//     // Check specific headers, source IP ranges, or HMAC signature.
//     Ok(())
// }

// Placeholder for storing content
// async fn store_ft_content_in_db(
//    db_pool: &web::Data<DbPool>,
//    content_uuid: Uuid,
//    content_data: &serde_json::Value
// ) -> Result<(), ApiError> {
//     // TODO: Implement Diesel logic to INSERT or UPDATE content in a dedicated table.
//     log::debug!("Storing FT content {} in DB (placeholder)", content_uuid);
//     Ok(())
// }