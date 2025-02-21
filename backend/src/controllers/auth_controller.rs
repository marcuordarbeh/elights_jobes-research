use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use crate::services::auth_service;

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
    role: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[post("/register")]
async fn register(req: web::Json<RegisterRequest>) -> HttpResponse {
    match auth_service::register(&req.username, &req.password, &req.role).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/login")]
async fn login(req: web::Json<LoginRequest>) -> HttpResponse {
    match auth_service::login(&req.username, &req.password).await {
        Ok(token) => HttpResponse::Ok().body(token),
        Err(err) => HttpResponse::Unauthorized().body(err.to_string()),
    }
}