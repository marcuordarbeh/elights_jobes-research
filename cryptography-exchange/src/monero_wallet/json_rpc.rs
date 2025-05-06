// /home/inno/elights_jobes-research/cryptography-exchange/src/monero_wallet/json_rpc.rs
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Generic JSON-RPC Request structure.
#[derive(Serialize, Debug)]
pub struct JsonRpcRequest<'a, T: Serialize> {
    jsonrpc: &'static str,
    id: &'static str, // Use fixed ID or generate unique ones if needed
    method: &'a str,
    params: T,
}

impl<'a, T: Serialize> JsonRpcRequest<'a, T> {
    pub fn new(method: &'a str, params: T) -> Self {
        JsonRpcRequest {
            jsonrpc: "2.0",
            id: "0", // Simple fixed ID
            method,
            params,
        }
    }
}

/// Generic JSON-RPC Response structure.
#[derive(Deserialize, Debug)]
pub struct JsonRpcResponse<T> {
    jsonrpc: String,
    id: String, // Should match request ID
    result: Option<T>,
    error: Option<JsonRpcError>,
}

/// JSON-RPC Error object.
#[derive(Deserialize, Debug)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    pub data: Option<JsonValue>, // Optional extra error info
}

impl<T> JsonRpcResponse<T> {
    /// Extracts the result or converts the error into crate's ExchangeError.
    pub fn into_result(self) -> Result<T, crate::ExchangeError> {
        match (self.result, self.error) {
            (Some(result), None) => Ok(result),
            (None, Some(error)) => Err(crate::ExchangeError::JsonRpcError {
                code: error.code,
                message: error.message,
            }),
            // Other cases (both Some, both None) are generally invalid JSON-RPC responses
            _ => Err(crate::ExchangeError::ResponseParseError(
                "Invalid JSON-RPC response structure".to_string(),
            )),
        }
    }
}


// --- Specific Parameter Structures for Monero RPC Methods ---

#[derive(Serialize, Debug, Default)]
pub struct GetBalanceParams {
    // Optional: specify account index if needed
    // pub account_index: Option<u32>,
}

#[derive(Serialize, Debug)]
pub struct GetAddressParams {
    pub account_index: u32, // Account index is required
    // Optional: specify list of subaddress indices if needed
    // pub address_index: Option<Vec<u32>>,
}

#[derive(Serialize, Debug)]
pub struct Destination {
    pub amount: u64, // Amount in atomic units
    pub address: String, // Destination Monero address
}

#[derive(Serialize, Debug)]
pub struct TransferParams {
    pub destinations: Vec<Destination>,
    pub account_index: Option<u32>, // Source account index (usually 0)
    // pub subaddr_indices: Option<Vec<u32>>, // Source subaddress indices
    pub priority: Option<u32>, // Transaction priority (0-3, 0 for default)
    pub mixin: Option<u32>, // Ring size (usually handled by wallet default, e.g., 10)
    pub unlock_time: Option<u64>, // Lock time (block height or timestamp)
    pub payment_id: Option<String>, // Optional payment ID (hex string)
    pub get_tx_key: Option<bool>, // Request transaction secret key
    pub get_tx_hex: Option<bool>, // Request raw transaction hex
    // Add other params like do_not_relay etc.
}[]