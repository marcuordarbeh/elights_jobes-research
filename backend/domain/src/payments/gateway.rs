// /home/inno/elights_jobes-research/backend/domain/src/payments/gateway.rs
use async_trait::async_trait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("Authentication failed with gateway")]
    AuthenticationError,
    #[error("Network error communicating with gateway: {0}")]
    NetworkError(String),
    #[error("Gateway rejected request: Status={status:?}, Code={code:?}, Message={message:?}")]
    RequestRejected {
        status: Option<u16>,
        code: Option<String>,
        message: Option<String>,
    },
    #[error("Failed to parse gateway response: {0}")]
    ParseError(String),
    #[error("Gateway configuration error: {0}")]
    ConfigurationError(String),
    #[error("Operation timed out")]
    Timeout,
    #[error("Internal gateway error: {0}")]
    InternalError(String),
}

// Details of the payment method provided to the gateway
#[derive(Debug, Serialize, Clone)]
pub enum PaymentMethodDetails {
    CardToken(String),         // Token representing card details (e.g., Stripe token)
    GatewayReference(String), // Reference to a previous transaction on the gateway
    RawCardDetails(RawCard), // Use with extreme caution - requires PCI compliance
    // Add others like BankAccountToken, etc.
}

// Use only if absolutely necessary and PCI compliant
#[derive(Debug, Serialize, Clone)]
pub struct RawCard {
    number: String,
    exp_month: u8,
    exp_year: u16,
    cvc: String,
}


// Intent of the payment request
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub enum PaymentIntent {
    Authorize, // Authorize funds only
    Capture,   // Capture previously authorized funds
    AuthorizeAndCapture, // Authorize and capture in one step (Sale)
    Refund,    // Refund a previous transaction
    Validate,  // Validate payment method without charging (e.g., $0 auth)
}

/// Request structure sent to the payment gateway.
#[derive(Debug, Serialize, Clone)]
pub struct PaymentGatewayRequest {
    pub amount: Decimal,
    pub currency: String, // ISO 4217
    pub payment_method: PaymentMethodDetails,
    pub intent: PaymentIntent,
    pub description: Option<String>,
    pub customer_id: Option<String>, // Gateway's customer identifier
    pub metadata: Option<serde_json::Value>, // Pass-through metadata
    // Add fields for idempotency keys, return URLs, etc.
    // idempotency_key: Option<String>,
}

/// Response structure received from the payment gateway.
#[derive(Debug, Deserialize, Clone)]
pub struct PaymentGatewayResponse {
    pub success: bool, // Simple success/failure flag
    pub gateway_transaction_id: String, // Gateway's unique ID for the operation
    pub status: Option<String>, // Gateway's specific status string
    pub error_code: Option<String>, // Gateway's error code if failed
    pub error_message: Option<String>, // Gateway's error message if failed
    // Optional details specific to the response (parsed from gateway JSON)
    pub details: Option<serde_json::Value>,
}

/// Trait defining the interface for interacting with a payment gateway.
#[async_trait]
pub trait PaymentGateway: Send + Sync { // Ensure Send+Sync for multi-threaded envs
    /// Submits a payment request (auth, capture, refund) to the gateway.
    async fn submit_payment(
        &self,
        request: PaymentGatewayRequest,
    ) -> Result<PaymentGatewayResponse, GatewayError>;

    /// Fetches the status of a previous transaction from the gateway.
    async fn get_transaction_status(
        &self,
        gateway_transaction_id: &str,
    ) -> Result<PaymentGatewayResponse, GatewayError>;

    // Add other methods if needed (e.g., create customer, manage payment methods)
    // async fn create_customer(&self, ...) -> Result<..., GatewayError>;
    // async fn tokenize_card(&self, ...) -> Result<String, GatewayError>;
}


// --- Mock Implementation for Testing ---

#[derive(Debug, Clone, Default)]
pub struct MockPaymentGateway {
    pub should_succeed: bool,
    pub simulate_delay_ms: Option<u64>,
}

#[async_trait]
impl PaymentGateway for MockPaymentGateway {
    async fn submit_payment(
        &self,
        request: PaymentGatewayRequest,
    ) -> Result<PaymentGatewayResponse, GatewayError> {
        if let Some(delay) = self.simulate_delay_ms {
            tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
        }

        let gateway_id = format!("MOCK_{}_{}",
            match request.intent {
                PaymentIntent::Authorize => "AUTH",
                PaymentIntent::Capture => "CAP",
                PaymentIntent::AuthorizeAndCapture => "SALE",
                PaymentIntent::Refund => "REF",
                PaymentIntent::Validate => "VAL",
            },
            rand::random::<u32>()
        );

        if self.should_succeed {
            Ok(PaymentGatewayResponse {
                success: true,
                gateway_transaction_id: gateway_id,
                status: Some(match request.intent { // Simulate typical success statuses
                    PaymentIntent::Authorize => "Authorized".to_string(),
                    PaymentIntent::Capture | PaymentIntent::AuthorizeAndCapture => "Succeeded".to_string(),
                    PaymentIntent::Refund => "Succeeded".to_string(),
                    PaymentIntent::Validate => "Validated".to_string(),
                }),
                error_code: None,
                error_message: None,
                details: Some(json!({"mock": true, "intent": request.intent})),
            })
        } else {
            Ok(PaymentGatewayResponse {
                success: false,
                gateway_transaction_id: gateway_id,
                status: Some("Failed".to_string()),
                error_code: Some("mock_fail_code".to_string()),
                error_message: Some("Mock Gateway Failure".to_string()),
                details: Some(json!({"mock": true, "intent": request.intent})),
            })
            // Or return Err(GatewayError::RequestRejected{...}) for failure simulation
        }
    }

     async fn get_transaction_status(
        &self,
        gateway_transaction_id: &str,
    ) -> Result<PaymentGatewayResponse, GatewayError> {
         if let Some(delay) = self.simulate_delay_ms {
            tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
        }
        // Simple mock: Assume if ID exists, it succeeded previously
         Ok(PaymentGatewayResponse {
            success: true,
            gateway_transaction_id: gateway_transaction_id.to_string(),
            status: Some("Succeeded".to_string()), // Assume succeeded if found
            error_code: None,
            error_message: None,
            details: Some(json!({"mock": true, "status_check": true})),
        })
    }
}
use serde_json::json; // Ensure this is imported