// /home/inno/elights_jobes-research/backend/core-api/src/handlers/auth.rs
use crate::db::{get_db_conn, DbPool};
use crate::error::{ApiError, internal_error};
use crate::models::{ApiAuthResponse, ApiLoginRequest, ApiRegisterRequest, ApiRegisterResponse};
use crate::config::AppConfig; // Import AppConfig
use actix_web::{web, HttpResponse, Responder};
use domain::security::auth::{authenticate_user, hash_password}; // Use domain auth functions
use domain::models::{NewUser}; // Use domain model for insertion
use std::sync::Arc; // For Arc<AppConfig>
use uuid::Uuid;

/// Handles user registration requests.
pub async fn register(
    db_pool: web::Data<DbPool>,
    info: web::Json<ApiRegisterRequest>,
) -> Result<impl Responder, ApiError> {
    log::info!("Registration attempt for user: {}", info.username);

    // Basic input validation
    if info.password.len() < 8 {
        return Err(ApiError::ValidationError("Password must be at least 8 characters".to_string()));
    }
    // TODO: Add email format validation

    // Hash password (CPU intensive, run in blocking thread)
    let password = info.password.clone();
    let password_hash = web::block(move || hash_password(&password))
        .await? // Handle blocking error -> InternalError
        .map_err(ApiError::DomainLogicError)?; // Handle DomainError

    // Insert user into database
    let mut conn = get_db_conn(&db_pool)?; // Get connection from pool
    let username = info.username.clone();
    let email = info.email.clone();

    // Use web::block for Diesel operation
    let result = web::block(move || {
        use crate::schema::users::dsl::*; // Import DSL for users table
        use diesel::RunQueryDsl;

        let new_db_user = NewUser { // Create struct matching Insertable
            username: &username,
            email: &email,
            password_hash: &password_hash,
        };

        // Insert and return the new user's ID
        diesel::insert_into(users)
            .values(&new_db_user)
            .returning(user_id) // Return the generated UUID
            .get_result::<Uuid>(&mut conn)
    })
    .await?; // Handle blocking error

    match result {
         Ok(new_user_id) => {
            log::info!("User '{}' registered successfully with ID: {}", info.username, new_user_id);
             // TODO: Log audit event via domain service
             Ok(HttpResponse::Created().json(ApiRegisterResponse {
                 message: "User registered successfully".to_string(),
                 user_id: new_user_id,
             }))
         }
        Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
             log::warn!("Registration failed for {}: Username or email already exists", info.username);
             Err(ApiError::BadRequest("Username or email already exists".to_string()))
         }
        Err(e) => {
            log::error!("Database error during registration for {}: {}", info.username, e);
            // Wrap the Diesel error into internal_error to hide details
            Err(internal_error(e))
        }
     }
}

/// Handles user login requests.
pub async fn login(
    db_pool: web::Data<DbPool>,
    app_config: web::Data<Arc<AppConfig>>, // Get config from app state
    info: web::Json<ApiLoginRequest>,
) -> Result<impl Responder, ApiError> {
    log::info!("Login attempt for user: {}", info.username);

    let mut conn = get_db_conn(&db_pool)?;
    let username = info.username.clone();
    let password = info.password.clone();
    let config = app_config.get_ref().clone(); // Clone Arc<AppConfig> for blocking thread

    // Fetch user and authenticate (potentially blocking)
    let auth_result = web::block(move || {
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;
        use domain::models::User as DomainUser; // Import domain User

        // 1. Fetch user by username
        let user = users
            .filter(username.eq(&username)) // Use cloned username
            .select(DomainUser::as_select()) // Select into domain::User struct
            .first::<DomainUser>(&mut conn)
            .optional()? // Handle NotFound gracefully
            .ok_or(domain::DomainError::Authentication("Invalid credentials".to_string()))?; // Return Auth error if not found

        // 2. Authenticate (Verify password and generate token)
        // Requires JWT feature enabled in domain crate for real tokens
        authenticate_user(
             &user.username,
             &password, // Use cloned password
             &user.password_hash,
             "user", // TODO: Get actual user role
             config.jwt_secret.as_bytes(),
             config.jwt_duration_hours,
        )
    })
    .await?; // Handle blocking error

    match auth_result {
        Ok(auth_token) => {
             log::info!("User '{}' logged in successfully.", info.username);
             // TODO: Log audit event
             Ok(HttpResponse::Ok().json(ApiAuthResponse {
                 token: auth_token.token,
                 expires_at: auth_token.expires_at,
             }))
        }
        Err(domain_err) => {
             // Map domain error to API error
             log::warn!("Authentication failed for {}: {}", info.username, domain_err);
             Err(ApiError::DomainLogicError(domain_err))
        }
    }
}