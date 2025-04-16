use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
}

pub async fn login(info: web::Json<LoginRequest>) -> HttpResponse {
    // Placeholder logic. Implement your authentication checks and JWT generation.
    println!("Attempting login for user: {}", info.username);
    HttpResponse::Ok().json(AuthResponse { token: "dummy_token".to_string() })
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
    email: String,
}

pub async fn register(info: web::Json<RegisterRequest>) -> HttpResponse {
    // Placeholder for user registration. Insert into DB, hash passwords, etc.
    println!("Registering new user: {}", info.username);
    HttpResponse::Created().json(AuthResponse { token: "dummy_token_after_registration".to_string() })
}
