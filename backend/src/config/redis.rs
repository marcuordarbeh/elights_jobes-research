// src/config/redis.rs - Initializes a Redis connection manager.
use redis::aio::ConnectionManager;
use redis::Client;
use std::env;

#[allow(dead_code)]
pub async fn init_redis() -> ConnectionManager {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".into());
    let client = Client::open(redis_url).expect("Failed to create Redis client");
    ConnectionManager::new(client)
        .await
        .expect("Failed to create Redis connection manager")
}

// The old implementation was removed.
// If needed later, add it as a separate function or proper module.