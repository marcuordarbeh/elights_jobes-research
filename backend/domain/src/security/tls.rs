// /home/inno/elights_jobes-research/backend/domain/src/security/tls.rs
use rustls::{Certificate, PrivateKey, ServerConfig, RootCertStore};
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys}; // Added pkcs8
use std::{fs::File, io::{self, BufReader}};
use std::sync::Arc;
use crate::error::DomainError; // Use domain error

/// Loads TLS server configuration from certificate and key files.
/// Supports RSA and PKCS8 private keys.
pub fn load_tls_config(cert_path: &str, key_path: &str) -> Result<ServerConfig, DomainError> {
    // Load certificate chain
    let cert_file = File::open(cert_path)
        .map_err(|e| DomainError::Configuration(format!("Failed to open cert file '{}': {}", cert_path, e)))?;
    let mut cert_reader = BufReader::new(cert_file);
    let cert_chain = certs(&mut cert_reader)
        .map_err(|e| DomainError::Configuration(format!("Failed to parse cert file '{}': {}", cert_path, e)))?
        .into_iter()
        .map(Certificate)
        .collect();

    // Load private key (try RSA then PKCS8)
    let key_file = File::open(key_path)
        .map_err(|e| DomainError::Configuration(format!("Failed to open key file '{}': {}", key_path, e)))?;
    let mut key_reader = BufReader::new(key_file);

    // Try PKCS8 first
    let mut keys = pkcs8_private_keys(&mut key_reader)
        .map_err(|e| DomainError::Configuration(format!("Failed to parse PKCS8 key file '{}': {}", key_path, e)))?;

    // If no PKCS8 keys found, reset reader and try RSA
    if keys.is_empty() {
         let key_file_reset = File::open(key_path) // Re-open file
            .map_err(|e| DomainError::Configuration(format!("Failed to re-open key file '{}': {}", key_path, e)))?;
         let mut key_reader_reset = BufReader::new(key_file_reset);
         keys = rsa_private_keys(&mut key_reader_reset)
             .map_err(|e| DomainError::Configuration(format!("Failed to parse RSA key file '{}': {}", key_path, e)))?;
    }


    // Extract the first valid key
    let key = keys.into_iter().next().map(PrivateKey)
         .ok_or_else(|| DomainError::Configuration(format!("No private key found in {}", key_path)))?;


    // Build ServerConfig
    ServerConfig::builder()
        .with_safe_defaults() // Use safe defaults for TLS protocols and cipher suites
        .with_no_client_auth() // Require no client certificate by default
        // .with_client_cert_verifier(build_verifier()) // Example: Enable mTLS
        .with_single_cert(cert_chain, key)
        .map_err(|e| DomainError::Configuration(format!("Failed to build TLS config: {}", e)))
}


// Optional: Function to build a client certificate verifier for Mutual TLS (mTLS)
// Requires a RootCertStore with trusted client CAs.
// fn build_verifier() -> Arc<dyn rustls::server::ClientCertVerifier> {
//     let mut root_store = RootCertStore::empty();
//     // TODO: Add trusted client CA certificates to root_store
//     // e.g., from a file:
//     // let ca_file = File::open("path/to/client_ca.pem").unwrap();
//     // let mut reader = BufReader::new(ca_file);
//     // root_store.add_pem_file(&mut reader).unwrap();
//
//     rustls::server::AllowAnyAuthenticatedClient::new(root_store)
// }