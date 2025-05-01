// /home/inno/elights_jobes-research/backend/core-api/src/handlers/auth.rs
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::error::ApiError;
// Assume DbPool is sqlx::PgPool, adjust if using Diesel
use sqlx::PgPool;
// Import domain functions (adjust path/names as needed)
use domain::security::auth::{authenticate_user, hash_password, AuthToken, UserCredentials};
use domain::models::user::User as DomainUser; // Example import, might need different model

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct AuthResponse {
    token: String,
}

#[derive(Debug, Serialize)]
struct RegisterResponse {
    message: String,
    username: String,
}


/// Handles user login requests.
pub async fn login(
    db_pool: web::Data<PgPool>,
    info: web::Json<LoginRequest>,
) -> Result<impl Responder, ApiError> {
    log::info!("Login attempt for user: {}", info.username);

    // --- Placeholder Logic ---
    // 1. Fetch user from database by username
    // Example using SQLx:
    let user_result = sqlx::query_as!(
        DomainUser, // Use your actual user struct compatible with sqlx::FromRow
        "SELECT username, email, password_hash, created_at FROM core_schema.users WHERE username = $1",
        info.username
    )
    .fetch_optional(db_pool.get_ref()) // Use get_ref() for web::Data<Pool>
    .await;

    let user = match user_result {
         Ok(Some(user)) => user,
         Ok(None) => return Err(ApiError::AuthenticationError("User not found".to_string())),
         Err(e) => {
             log::error!("Database error during login: {}", e);
             return Err(ApiError::DatabaseError("Failed to fetch user".to_string()));
         }
    };

    // 2. Verify password and generate token (using domain function)
    // Retrieve JWT secret from environment securely
    let jwt_secret = std::env::var("JWT_SECRET").map_err(|_| ApiError::ConfigurationError("JWT_SECRET not set".to_string()))?;

    let credentials = UserCredentials {
        username: &info.username,
        password: &info.password,
    };

    // Assuming authenticate_user is adapted for async or is synchronous
    // You might need to spawn_blocking if bcrypt/jwt calls are CPU-intensive and sync
    match authenticate_user(&credentials, &user.password_hash, "user", jwt_secret.as_bytes()) { // Assuming role "user" for now
        Ok(auth_token) => Ok(HttpResponse::Ok().json(AuthResponse { token: auth_token.token })),
        Err(domain_err) => {
            log::warn!("Authentication failed for {}: {}", info.username, domain_err);
             match domain_err {
                domain::DomainError::Security(_) => Err(ApiError::AuthenticationError("Invalid credentials".to_string())),
                _ => Err(ApiError::InternalError("Authentication process failed".to_string())),
            }
        }
    }
    // --- End Placeholder Logic ---
}

/// Handles new user registration requests.
pub async fn register(
    db_pool: web::Data<PgPool>,
    info: web::Json<RegisterRequest>,
) -> Result<impl Responder, ApiError> {
    log::info!("Registration attempt for user: {}", info.username);

    // --- Placeholder Logic ---
    // 1. Validate input (e.g., password strength, email format - can be in domain)
    if info.password.len() < 8 {
        return Err(ApiError::ValidationError("Password must be at least 8 characters".to_string()));
    }
    // Add email format validation

    // 2. Hash the password (using domain function)
    let password_hash = hash_password(&info.password)?; // Propagate DomainError via ApiError::DomainLogicError

    // 3. Insert the new user into the database
    // Example using SQLx:
    let insert_result = sqlx::query!(
        "INSERT INTO core_schema.users (username, email, password_hash) VALUES ($1, $2, $3)",
        info.username,
        info.email,
        password_hash
    )
    .execute(db_pool.get_ref())
    .await;

    match insert_result {
        Ok(result) if result.rows_affected() == 1 => {
            log::info!("User '{}' registered successfully", info.username);
             Ok(HttpResponse::Created().json(RegisterResponse {
                 message: "User registered successfully".to_string(),
                 username: info.username.clone(),
            }))
        }
        Ok(_) => {
            // Should not happen if INSERT was successful but affected 0 rows
            log::error!("User registration inserted 0 rows for {}", info.username);
            Err(ApiError::InternalError("User registration failed unexpectedly".to_string()))
        }
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
             log::warn!("Registration failed for {}: Username or email already exists", info.username);
             Err(ApiError::BadRequest("Username or email already exists".to_string()))
         }
        Err(e) => {
            log::error!("Database error during registration for {}: {}", info.username, e);
            Err(ApiError::DatabaseError("Failed to register user".to_string()))
        }
    }
    // --- End Placeholder Logic ---
}