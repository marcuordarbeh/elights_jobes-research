// /home/inno/elights_jobes-research/backend/core-api/src/services/ft_client.rs
use crate::error::{ApiError, internal_error};
use reqwest::{Client as HttpClient, Method, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;

const FT_API_BASE_URL: &str = "https://api.ft.com"; // Confirm base URL from FT docs

#[derive(Clone)] // Make client cloneable for sharing via web::Data
pub struct FtApiClient {
    http_client: HttpClient,
    api_key: String,
}

impl FtApiClient {
    pub fn new(api_key: String, http_client: HttpClient) -> Self {
        FtApiClient { http_client, api_key }
    }

    /// Helper to build authenticated requests for FT API.
    fn build_request(&self, method: Method, endpoint: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", FT_API_BASE_URL, endpoint);
        self.http_client
            .request(method, url)
            .header("X-Api-Key", &self.api_key) // Common header for API keys
            .header(reqwest::header::ACCEPT, "application/json")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
    }

    /// Helper to handle FT API response status and parsing.
    async fn handle_ft_response<T: DeserializeOwned>(
        &self,
        operation_name: &str,
        response: reqwest::Response,
    ) -> Result<T, ApiError> {
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            log::error!("FT API Error ({}) - Status: {}, Body: {}", operation_name, status, body);
            // Map specific FT error codes if documented
            let api_error = match status {
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => ApiError::AuthenticationError("Invalid FT API Key or insufficient permissions".to_string()),
                StatusCode::NOT_FOUND => ApiError::NotFound(format!("FT API resource not found ({})", operation_name)),
                StatusCode::TOO_MANY_REQUESTS => ApiError::ExternalServiceError("FT API rate limit exceeded".to_string()),
                _ => ApiError::ExternalServiceError(format!("FT API failed with status {}: {}", status, body)),
            };
            return Err(api_error);
        }
        // Assuming success means JSON response
        response.json::<T>().await.map_err(|e| {
            log::error!("Failed to parse FT API response ({}): {}", operation_name, e);
            internal_error(e) // Wrap parsing error as internal
        })
    }


    // === FT API Methods ===

    /// Fetches content by its UUID.
    /// Endpoint: GET /content/{uuid}
    pub async fn get_content(&self, content_uuid: &str) -> Result<JsonValue, ApiError> {
        let endpoint = format!("/content/{}", content_uuid);
        log::debug!("FT Client: Getting content {}", content_uuid);
        let request = self.build_request(Method::GET, &endpoint);
        let response = request.send().await?;
        self.handle_ft_response("GetContent", response).await
    }

    // TODO: Add methods for managing push subscriptions if needed
    // E.g., POST /push/subscriptions
    // pub async fn create_push_subscription(&self, ...) -> Result<..., ApiError> { ... }

    // TODO: Add methods for searching content if needed
    // E.g., POST /content/search/v1
    // pub async fn search_content(&self, ...) -> Result<..., ApiError> { ... }

}