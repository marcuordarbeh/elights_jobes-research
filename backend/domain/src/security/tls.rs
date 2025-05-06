// /home/inno/elights_jobes-research/backend/domain/src/security/tls.rs
use rustls::{Certificate, PrivateKey, ServerConfig}; // Removed RootCertStore for basic server config
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use std::{fs::File, io::{self, BufReader}};
use std::sync::Arc; // Keep Arc for ServerConfig
use crate::error::DomainError;

/// Loads TLS server configuration from certificate and key files.
/// Supports RSA and PKCS8 private keys.
pub fn load_tls_config(cert_path: &str, key_path: &str) -> Result<Arc<ServerConfig>, DomainError> { // Return Arc<ServerConfig>
    log::info!("Loading TLS config: cert='{}', key='{}'", cert_path, key_path);
    // Load certificate chain
    let cert_file = File::open(cert_path)
        .map_err(|e| DomainError::Configuration(format!("Failed to open cert file '{}': {}", cert_path, e)))?;
    let mut cert_reader = BufReader::new(cert_file);
    // Use .collect() to handle potential errors during parsing
    let cert_chain: Vec<Certificate> = certs(&mut cert_reader)
        .map_err(|e| DomainError::Configuration(format!("Failed to parse cert file '{}': {}", cert_path, e)))?
        .into_iter()
        .map(Certificate)
        .collect();

    if cert_chain.is_empty() {
        return Err(DomainError::Configuration(format!("No certificates found in {}", cert_path)));
    }

    // Load private key (try PKCS8 then RSA)
    let key_file = File::open(key_path)
        .map_err(|e| DomainError::Configuration(format!("Failed to open key file '{}': {}", key_path, e)))?;
    let mut key_reader = BufReader::new(key_file);

    let mut keys = pkcs8_private_keys(&mut key_reader)
        .map_err(|e| DomainError::Configuration(format!("Failed to parse PKCS8 key file '{}': {}", key_path, e)))?;

    if keys.is_empty() {
        log::debug!("No PKCS8 keys found in '{}', trying RSA...", key_path);
        // Reset reader by reopening the file
        let key_file_reset = File::open(key_path)
           .map_err(|e| DomainError::Configuration(format!("Failed to re-open key file '{}': {}", key_path, e)))?;
        let mut key_reader_reset = BufReader::new(key_file_reset);
        keys = rsa_private_keys(&mut key_reader_reset)
            .map_err(|e| DomainError::Configuration(format!("Failed to parse RSA key file '{}': {}", key_path, e)))?;
    }

    // Extract the first valid key
    let key = keys.into_iter().next().map(PrivateKey)
        .ok_or_else(|| DomainError::Configuration(format!("No private key found in {}", key_path)))?;

    // Build ServerConfig
    let config = ServerConfig::builder()
        .with_safe_defaults() // Use recommended cipher suites and protocols
        .with_no_client_auth() // Change if client auth (mTLS) is needed
        .with_single_cert(cert_chain, key)
        .map_err(|e| DomainError::Configuration(format!("Failed to build TLS ServerConfig: {}", e)))?;

    log::info!("TLS ServerConfig loaded successfully.");
    Ok(Arc::new(config)) // Return Arc<ServerConfig>
}