// /home/inno/elights_jobes-research/backend/core-api/src/middlewares/auth_guard.rs
use crate::config::AppConfig;
use crate::error::ApiError;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderValue, AUTHORIZATION},
    Error as ActixError, HttpMessage, // Import Actix Error type
};
use futures::future::{ok, Ready, LocalBoxFuture};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll};
use uuid::Uuid;

// Structure to hold authenticated user claims, added to request extensions
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid, // Assuming user ID is stored in DB as UUID
    pub username: String, // Subject from JWT
    pub role: String, // Role from JWT
}

// --- Middleware Factory ---
#[derive(Clone)]
pub struct AuthGuard;

impl<S, B> Transform<S, ServiceRequest> for AuthGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type InitError = ();
    type Transform = AuthGuardMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthGuardMiddleware {
            service: Rc::new(service),
        })
    }
}

// --- Middleware Implementation ---
pub struct AuthGuardMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError; // Middleware errors must be ActixError
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Clone needed data before moving into async block
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // 1. Extract AppConfig (contains JWT secret) from application data
            // If AppConfig is not found, it's an internal server error (configuration issue)
            let config = req.app_data::<web::Data<Arc<AppConfig>>>().ok_or_else(|| {
                log::error!("AuthGuard: AppConfig not found in application data.");
                ApiError::ConfigurationError("App configuration missing".to_string())
            })?.get_ref().clone(); // Clone Arc for potential use

            // 2. Extract Authorization header
            let auth_header = req.headers().get(AUTHORIZATION);

            // 3. Validate Token
            match validate_auth_header(auth_header, &config.jwt_secret) {
                Ok(claims) => {
                     // 4. Fetch User ID from DB (or assume sub claim *is* the UUID)
                     // This step depends on whether sub claim holds username or UUID directly.
                     // Assuming 'sub' holds username for now, requires DB lookup.
                     // DB lookups in middleware can add latency. Consider if sub should be UUID.
                     // For simplicity, let's assume sub is parseable as UUID directly here.
                     // let user_id = lookup_user_id_by_username(&db_pool, &claims.sub).await?;
                    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
                         log::error!("AuthGuard: Failed to parse user ID (sub claim) '{}' as UUID.", &claims.sub);
                         ApiError::AuthenticationError("Invalid user identifier in token".to_string())
                     })?;

                    // 5. Insert AuthenticatedUser claims into request extensions
                    let authenticated_user = AuthenticatedUser {
                        user_id,
                        username: claims.sub, // Keep username as well
                        role: claims.role,
                    };
                    req.extensions_mut().insert(authenticated_user); // Make claims available to handlers

                    log::debug!("AuthGuard: Authentication successful for user {}", user_id);

                    // 6. Forward request to the next service
                    let fut = service.call(req);
                    fut.await
                }
                Err(api_error) => {
                    log::warn!("AuthGuard: Authentication failed: {}", api_error);
                    // Convert ApiError into ActixError before returning
                    Err(actix_web::Error::from(api_error))
                }
            }
        })
    }
}


/// Helper function to validate the Authorization header and JWT token.
fn validate_auth_header(
    header: Option<&HeaderValue>,
    jwt_secret: &str,
) -> Result<domain::security::auth::Claims, ApiError> {
    let header_value = header.ok_or(ApiError::AuthenticationError("Missing Authorization header".to_string()))?;
    let header_str = header_value.to_str().map_err(|_| {
        ApiError::BadRequest("Invalid Authorization header format".to_string())
    })?;

    // Check for "Bearer " prefix
    let prefix = "Bearer ";
    if !header_str.starts_with(prefix) {
        return Err(ApiError::AuthenticationError("Invalid token scheme, expected Bearer".to_string()));
    }
    let token = &header_str[prefix.len()..];

    // Validate the token using the domain function
    // Requires jwt_support feature in domain crate
    #[cfg(feature = "jwt_support")]
    let claims = domain::security::auth::validate_token(token, jwt_secret.as_bytes())
        .map_err(ApiError::DomainLogicError)?; // Wrap DomainError

    // Dummy validation if JWT feature not compiled in domain
    #[cfg(not(feature = "jwt_support"))]
    let claims = domain::security::auth::validate_token(token, jwt_secret.as_bytes())
         .map_err(ApiError::DomainLogicError)?;

    Ok(claims)
}

// Optional: DB lookup function (would need db_pool passed or available)
// async fn lookup_user_id_by_username(db_pool: &DbPool, username: &str) -> Result<Uuid, ApiError> {
//     let mut conn = get_db_conn(&web::Data::new(db_pool.clone()))?; // Example getting connection
//     web::block(move || {
//         use crate::schema::users::dsl::*;
//         users.filter(username.eq(username)).select(user_id).first(&mut conn)
//     })
//     .await?
//     .map_err(|e| ApiError::AuthenticationError("User lookup failed".to_string()))
// }

use actix_web::web; // Required for web::Data in helpers