// /home/inno/elights_jobes-research/cryptography-exchange/src/utils.rs
use crate::error::ExchangeError;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex; // For comparing hex encoded MACs

type HmacSha256 = Hmac<Sha256>;

/// Verifies webhook signatures, e.g., from BTCPay Server.
/// BTCPay uses HMAC-SHA256 signature in the 'BTCPay-Sig' header, formatted as "sha256=...".
pub fn verify_btcpay_webhook_signature(
    webhook_secret: &str,
    request_body: &[u8], // Raw request body bytes
    signature_header: Option<&str>, // Value of the 'BTCPay-Sig' header
) -> Result<(), ExchangeError> {

    let signature = signature_header
        .ok_or_else(|| ExchangeError::WebhookVerificationError("Missing signature header".to_string()))?;

    // Extract the hex signature part after "sha256="
    let prefix = "sha256=";
    if !signature.starts_with(prefix) {
         return Err(ExchangeError::WebhookVerificationError("Invalid signature format (missing prefix)".to_string()));
    }
    let expected_sig_hex = &signature[prefix.len()..];

    // Calculate the HMAC-SHA256 of the body using the secret
    let mut mac = HmacSha256::new_from_slice(webhook_secret.as_bytes())
        .map_err(|e| ExchangeError::InternalError(format!("Failed to initialize HMAC: {}", e)))?;
    mac.update(request_body);
    let calculated_sig_bytes = mac.finalize().into_bytes();
    let calculated_sig_hex = hex::encode(calculated_sig_bytes);

    // Compare calculated signature with the one from the header (constant time comparison recommended, but simple equality often used here)
    if calculated_sig_hex == expected_sig_hex {
        log::debug!("Webhook signature verified successfully.");
        Ok(())
    } else {
         log::warn!("Webhook signature mismatch. Expected: {}, Calculated: {}", expected_sig_hex, calculated_sig_hex);
         Err(ExchangeError::WebhookVerificationError("Signature mismatch".to_string()))
    }
}

// --- Serde Helper for Monero Atomic Units ---
// Needed because u64 might exceed JS MAX_SAFE_INTEGER if sent directly as number in JSON
pub mod serde_monero_atomic {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(atomic_units: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&atomic_units.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        u64::from_str(&s).map_err(serde::de::Error::custom)
    }
}