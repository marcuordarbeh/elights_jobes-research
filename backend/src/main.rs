use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod config;
mod controllers;
mod middlewares;
mod models;
mod repositories;
mod services;
mod utils;
mod config::redis as redis_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = config::db::init_pool(&database_url).await;
    
    // Initialize Redis connection
    let redis_conn = redis_config::init_redis().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(redis_conn.clone()))
            .wrap(middlewares::auth::AuthMiddleware)
            .configure(controllers::init_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
