use actix_web::{App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::env;

// Import route initializers
mod routes;
mod config;
mod middlewares;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    env_logger::init();
    
    let server_address = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    
    println!("🚀 Starting server at http://{}", server_address);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // Using built-in logger middleware
            .configure(routes::auth::init)
            .configure(routes::payments::init)
            .configure(routes::crypto::init)
            .configure(routes::conversion::init)
    })
    .bind(server_address)?
    .run()
    .await
}
