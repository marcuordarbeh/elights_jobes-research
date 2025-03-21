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
pub async fn register(
    req: web::Json<RegisterRequest>,
    db_pool: web::Data<sqlx::PgPool>,
) -> HttpResponse {
    match auth_service::register(&db_pool, &req.username, &req.password, &req.role).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[post("/login")]
pub async fn login(
    req: web::Json<LoginRequest>,
    db_pool: web::Data<sqlx::PgPool>,
) -> HttpResponse {
    match auth_service::login(&db_pool, &req.username, &req.password).await {
        Ok(token) => HttpResponse::Ok().body(token),
        Err(err) => HttpResponse::Unauthorized().body(err),
    }
}
