use actix_web::{App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::env;

use crate::config::ft_asset::FtAssetConfig;
use crate::config::bank_server::BankServerConfig;
use crate::middlewares::ip_whitelist::ip_whitelist_middleware;

// Import route initializers
mod routes;
mod config;
mod middlewares;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ft_cfg = FtAssetConfig::from_env();
    let bs_cfg = BankServerConfig::from_env();

    // Load environment variables from .env file
    dotenv().ok();
    env_logger::init();
    
    let server_address = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    
    println!("🚀 Starting server at http://{}", server_address);

    HttpServer::new(move || {
        App::new()
            .wrap(ip_whitelist_middleware(ft_cfg.allowed_ips.clone())) // whitelist FT IPs
            .wrap(Logger::default()) // Using built-in logger middleware
            .configure(routes::auth::init)
            .configure(routes::payments::init)
            .configure(routes::crypto::init)
            .configure(routes::conversion::init)
            .service(web::scope("/api")
                .configure(routes::payments)
                .configure(routes::conversion)
                /* ... */
            )
    })
    .bind(server_address)?
    .bind((bs_cfg.host.clone(), bs_cfg.port))?
    .run()
    .await
}
