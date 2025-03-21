use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::models::{Claims};
use crate::repositories::user_repository;
use sqlx::PgPool;
use std::env;

pub async fn register(
    pool: &PgPool,
    username: &str,
    password: &str,
    role: &str,
) -> Result<(), String> {
    let hashed_password = hash(password, DEFAULT_COST).map_err(|e| e.to_string())?;
    user_repository::create_user(pool, username, &hashed_password, role)
        .await
        .map_err(|e| e.to_string())
}

pub async fn login(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<String, String> {
    let user = user_repository::find_user_by_username(pool, username)
        .await
        .map_err(|e| e.to_string())?;
    if verify(password, &user.password).map_err(|e| e.to_string())? {
        let claims = Claims {
            sub: user.id,
            role: user.role,
            exp: 10000000000,  // Adjust expiration as needed
        };
        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
        encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref()))
            .map_err(|e| e.to_string())
    } else {
        Err("Invalid credentials".into())
    }
}
