use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, rsa_private_keys};
use std::{fs::File, io::BufReader};

pub fn load_tls_config(cert_path: &str, key_path: &str) -> ServerConfig {
    let cert_file = &mut BufReader::new(File::open(cert_path).unwrap());
    let key_file = &mut BufReader::new(File::open(key_path).unwrap());

    let cert_chain = certs(cert_file).unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = rsa_private_keys(key_file).unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();
    let key = keys.remove(0);

    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()           // or with client auth for mutual TLS :contentReference[oaicite:10]{index=10}
        .with_single_cert(cert_chain, key)
        .expect("failed to build TLS config")
}
