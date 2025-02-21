use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};
use sqlx::PgPool;
use crate::models::{User, Claims};
use std::env;
use crate::repositories::user_repository;

const JWT_SECRET: &[u8] = b"secret";

pub async fn register(username: &str, password: &str, role: &str) -> Result<(), String> {
    let hashed_password = hash(password, DEFAULT_COST).map_err(|e| e.to_string())?;
    user_repository::create_user(username, &hashed_password, role).await.map_err(|e| e.to_string())
}

pub async fn login(username: &str, password: &str) -> Result<String, String> {
    let user = user_repository::find_user_by_username(username).await.map_err(|e| e.to_string())?;
    if verify(password, &user.password).map_err(|e| e.to_string())? {
        let claims = Claims {
            sub: user.id,
            role: user.role,
            exp: 10000000000,
        };
        encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
            .map_err(|e| e.to_string())
    } else {
        Err("Invalid credentials".to_string())
    }
}
