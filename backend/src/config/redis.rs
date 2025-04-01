// src/config/redis.rs - Initializes a Redis connection manager.
use redis::aio::ConnectionManager;
use redis::Client;
use std::env;

// In src/config/redis.rs:
#[allow(dead_code)]
pub async fn init_redis() -> ConnectionManager {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".into());
    let client = Client::open(redis_url).expect("Failed to create Redis client");
    ConnectionManager::new(client)
        .await
        .expect("Failed to create Redis connection manager")

        
// pub async fn init_redis() -> ConnectionManager {
//     let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".into());
//     let client = Client::open(redis_url).expect("Failed to create Redis client");
//     ConnectionManager::new(client)
//         .await
//         .expect("Failed to create Redis connection manager")
// }
