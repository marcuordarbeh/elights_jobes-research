// /home/inno/elights_jobes-research/backend/domain/src/security/hashing.rs
use crate::error::DomainError;
use sha2::{Sha256, Digest};
use base64::{engine::general_purpose::STANDARD as Base64Standard, Engine as _}; // Use standard Base64 engine

// TODO: Consider using a dedicated, peppered hashing scheme for sensitive data like account numbers
// instead of just SHA256, for better protection against rainbow tables if hashes leak.
// Libraries like `argon2` could be used, similar to passwords, but might be overkill.

/// Hashes sensitive data (like account numbers, IBANs) using SHA-256
/// and returns a Base64 encoded representation of the hash.
/// This is for storage and comparison, not encryption (data cannot be recovered).
pub fn hash_sensitive_data(data: &str) -> Result<String, DomainError> {
    if data.is_empty() {
        // Decide policy: error or return hash of empty string? Returning error is safer.
        return Err(DomainError::Validation("Cannot hash empty data".to_string()));
    }
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let hash_bytes = hasher.finalize();

    // Encode the raw hash bytes using Base64 for easier storage/representation in DB Text field
    let base64_hash = Base64Standard.encode(hash_bytes);

    Ok(base64_hash)
}

/// Verifies if provided data matches a stored SHA-256 Base64 hash.
pub fn verify_sensitive_data(data: &str, stored_base64_hash: &str) -> Result<bool, DomainError> {
     let new_hash = hash_sensitive_data(data)?;
     // Constant-time comparison is generally not strictly necessary for comparing hashes like this,
     // unlike password hashes where timing attacks might be a concern. Simple equality is usually fine.
     Ok(new_hash == stored_base64_hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashing_and_verification() {
        let data = "1234567890";
        let hash = hash_sensitive_data(data).unwrap();
        println!("Data: {}, Hash: {}", data, hash);

        assert!(verify_sensitive_data(data, &hash).unwrap());
        assert!(!verify_sensitive_data("0987654321", &hash).unwrap());
    }

     #[test]
    fn test_empty_data_hashing() {
        assert!(hash_sensitive_data("").is_err());
         // assert!(verify_sensitive_data("", "some_hash").is_err()); // verify depends on hash succeeding
    }
}