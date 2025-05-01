// /home/inno/elights_jobes-research/backend/domain/src/security/auth.rs
use serde::{Deserialize, Serialize};
use crate::error::DomainError;
// Add dependency for password hashing, e.g., bcrypt = "0.14"
use bcrypt::{hash, verify, DEFAULT_COST};

// Represents an authentication token (e.g., JWT)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthToken {
    pub token: String,
    // Optionally add expiry, refresh token, etc.
}

// Represents user credentials for authentication
#[derive(Debug, Deserialize)]
pub struct UserCredentials<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

// Represents claims within a JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (e.g., username)
    pub role: String, // User role for authorization
    pub exp: usize, // Expiration timestamp (Unix epoch)
}


/// Hashes a plain text password.
/// Requires a crate like `bcrypt`.
pub fn hash_password(password: &str) -> Result<String, DomainError> {
    hash(password, DEFAULT_COST)
        .map_err(|e| DomainError::Security(format!("Password hashing failed: {}", e)))
}

/// Verifies a plain text password against a stored hash.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, DomainError> {
    verify(password, hash)
        .map_err(|e| DomainError::Security(format!("Password verification failed: {}", e)))
}


/// Authenticates a user based on credentials and returns a token upon success.
/// Placeholder: Needs integration with user data store and JWT library.
pub fn authenticate_user(
    creds: &UserCredentials,
    stored_password_hash: &str, // Fetch this from the database for the user
    user_role: &str, // Fetch this from the database or derive it
    jwt_secret: &[u8], // Secret key for signing JWT
) -> Result<AuthToken, DomainError> {

    if verify_password(creds.password, stored_password_hash)? {
        // Password matches, generate JWT
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24)) // Token valid for 24 hours
            .expect("valid Timestamp")
            .timestamp();

        let claims = Claims {
            sub: creds.username.to_owned(),
            role: user_role.to_owned(),
            exp: expiration as usize,
        };

        // TODO: Use a JWT library (e.g., `jsonwebtoken`) to encode the claims
        // Example placeholder using jsonwebtoken = "8"
        // let header = jsonwebtoken::Header::default();
        // let encoding_key = jsonwebtoken::EncodingKey::from_secret(jwt_secret);
        // let token = jsonwebtoken::encode(&header, &claims, &encoding_key)
        //     .map_err(|e| DomainError::Security(format!("JWT generation failed: {}", e)))?;

        let dummy_token = format!("dummy_jwt_for_{}", creds.username); // Placeholder token

        Ok(AuthToken { token: dummy_token })
    } else {
        Err(DomainError::Security("Invalid credentials".to_string()))
    }
}

/// Validates an authentication token (e.g., JWT).
/// Placeholder: Needs integration with JWT library.
pub fn validate_token(token: &str, jwt_secret: &[u8]) -> Result<Claims, DomainError> {
    // TODO: Use a JWT library (e.g., `jsonwebtoken`) to decode and validate the token
    // Example placeholder using jsonwebtoken = "8"
    // let decoding_key = jsonwebtoken::DecodingKey::from_secret(jwt_secret);
    // let validation = jsonwebtoken::Validation::default(); // Add algorithm validation etc.
    // let token_data = jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation)
    //     .map_err(|e| DomainError::Security(format!("Invalid token: {}", e)))?;
    // Ok(token_data.claims)

    // Dummy validation
    if token.starts_with("dummy_jwt_for_") {
         let username = token.trim_start_matches("dummy_jwt_for_").to_string();
         Ok(Claims { sub: username, role: "user".to_string(), exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize })
    } else {
         Err(DomainError::Security("Invalid token (dummy check)".to_string()))
    }
}