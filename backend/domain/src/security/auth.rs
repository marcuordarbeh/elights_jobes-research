// /home/inno/elights_jobes-research/backend/domain/src/security/auth.rs
use serde::{Deserialize, Serialize};
use crate::error::DomainError;
use bcrypt::{hash, verify, DEFAULT_COST, BcryptError};
use chrono::{Utc, Duration};

// Represents an authentication token (e.g., JWT)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AuthToken {
    pub token: String,
    pub expires_at: i64, // Unix timestamp (seconds)
    // Optionally add refresh token, token type etc.
    // pub token_type: String, // e.g., "Bearer"
}

// Represents user credentials for authentication
#[derive(Debug, Deserialize)]
pub struct UserCredentials<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

// Represents claims within a JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,  // Subject (e.g., username or user UUID)
    pub role: String, // User role for authorization
    pub exp: i64,     // Expiration timestamp (Unix epoch seconds)
    pub iat: i64,     // Issued at timestamp
    // Add other custom claims as needed (e.g., session ID)
    // pub sid: Option<String>,
}


/// Hashes a plain text password using bcrypt.
pub fn hash_password(password: &str) -> Result<String, DomainError> {
    log::debug!("Hashing password...");
    // Run hashing in a blocking thread as it's CPU-intensive
    // Requires `tokio` in scope if used in async context: `tokio::task::spawn_blocking`
    // For simplicity here, assume it's called from sync context or handled appropriately upstream.
    hash(password, DEFAULT_COST)
        .map_err(|e| DomainError::Security(format!("Password hashing failed: {}", e)))
}

/// Verifies a plain text password against a stored bcrypt hash.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, DomainError> {
    log::debug!("Verifying password...");
     // Run verification in a blocking thread as it's CPU-intensive
    verify(password, hash)
         .map_err(|e| match e {
             // Handle specific error case where hash format is invalid vs. actual mismatch
             BcryptError::InvalidHash(_) => DomainError::Security(format!("Invalid password hash format provided: {}", e)),
             _ => DomainError::Security(format!("Password verification process error: {}", e)), // Other errors (cost mismatch?)
         })
}


/// Authenticates a user based on credentials and returns a token upon success.
/// Needs JWT Secret from configuration.
#[cfg(feature = "jwt_support")] // Only compile if jwt_support feature is enabled
pub fn authenticate_user(
    username: &str,          // Identifier used for login (could be email)
    provided_password: &str,
    stored_password_hash: &str, // Hash fetched from DB
    user_role: &str,         // Role fetched from DB or derived
    jwt_secret: &[u8],       // Secret key for signing JWT
    token_duration_hours: u32, // Token validity duration
) -> Result<AuthToken, DomainError> {

    if verify_password(provided_password, stored_password_hash)? {
        // Password matches, generate JWT
        let now = Utc::now();
        let iat = now.timestamp();
        let expires_at = (now + Duration::hours(token_duration_hours as i64)).timestamp();

        let claims = Claims {
            sub: username.to_owned(), // Use username or user_id as subject
            role: user_role.to_owned(),
            exp: expires_at,
            iat,
        };

        log::debug!("Generating JWT for user: {}", username);
        let header = jsonwebtoken::Header::default(); // Default is HS256
        let encoding_key = jsonwebtoken::EncodingKey::from_secret(jwt_secret);
        let token = jsonwebtoken::encode(&header, &claims, &encoding_key)
            .map_err(|e| {
                log::error!("JWT generation failed: {}", e);
                DomainError::Security(format!("Could not generate authentication token: {}", e))
            })?;

        Ok(AuthToken { token, expires_at })
    } else {
        log::warn!("Invalid password attempt for user: {}", username);
        Err(DomainError::Authentication("Invalid credentials".to_string()))
    }
}

/// Validates an authentication token (e.g., JWT).
/// Needs JWT Secret from configuration.
#[cfg(feature = "jwt_support")] // Only compile if jwt_support feature is enabled
pub fn validate_token(token: &str, jwt_secret: &[u8]) -> Result<Claims, DomainError> {
    log::debug!("Validating token...");
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(jwt_secret);
    // Default validation checks expiration (exp) and signature
    let validation = jsonwebtoken::Validation::default();
    // TODO: Add algorithm validation if not using default HS256
    // validation.algorithms = vec![jsonwebtoken::Algorithm::HS256];
    // TODO: Add audience (aud) or issuer (iss) validation if applicable

    let token_data = jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| {
            log::warn!("Token validation failed: {}", e);
            match e.kind() {
                 jsonwebtoken::errors::ErrorKind::ExpiredSignature => DomainError::Authentication("Token has expired".to_string()),
                 jsonwebtoken::errors::ErrorKind::InvalidToken
                 | jsonwebtoken::errors::ErrorKind::InvalidSignature
                 | jsonwebtoken::errors::ErrorKind::InvalidAlgorithm
                 | jsonwebtoken::errors::ErrorKind::InvalidAudience
                 | jsonwebtoken::errors::ErrorKind::InvalidIssuer
                 => DomainError::Authentication("Invalid token".to_string()),
                 _ => DomainError::Security(format!("Token validation error: {}", e)),
            }
        })?;

    log::debug!("Token validated successfully for user: {}", token_data.claims.sub);
    Ok(token_data.claims)
}

// --- Provide dummy implementations if JWT feature not enabled ---
#[cfg(not(feature = "jwt_support"))]
pub fn authenticate_user(
     username: &str, provided_password: &str, stored_password_hash: &str, user_role: &str,
     _jwt_secret: &[u8], token_duration_hours: u32
) -> Result<AuthToken, DomainError> {
     if verify_password(provided_password, stored_password_hash)? {
          let expires_at = (Utc::now() + Duration::hours(token_duration_hours as i64)).timestamp();
          log::warn!("JWT feature not enabled, returning dummy token for {}", username);
          Ok(AuthToken { token: format!("dummy_token_for_{}", username), expires_at })
     } else {
          Err(DomainError::Authentication("Invalid credentials".to_string()))
     }
}

#[cfg(not(feature = "jwt_support"))]
pub fn validate_token(token: &str, _jwt_secret: &[u8]) -> Result<Claims, DomainError> {
    log::warn!("JWT feature not enabled, performing dummy validation for token: {}", token);
     if token.starts_with("dummy_token_for_") {
         let username = token.trim_start_matches("dummy_token_for_").to_string();
         let now = Utc::now();
         Ok(Claims {
             sub: username,
             role: "user".to_string(), // Default dummy role
             exp: (now + Duration::hours(1)).timestamp(),
             iat: now.timestamp(),
         })
     } else {
          Err(DomainError::Authentication("Invalid dummy token".to_string()))
     }
}