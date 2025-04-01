// redis.rs - Initializes a Redis connection manager.
use redis::aio::ConnectionManager;
use redis::Client;
use std::env;

use redis::Client;

async fn connect_redis() -> redis::aio::Connection {
    let client = Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
    client.get_async_connection().await.expect("Failed to get Redis connection")
}      
// pub async fn init_redis() -> ConnectionManager {
//     let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".into());
//     let client = Client::open(redis_url).expect("Failed to create Redis client");
//     ConnectionManager::new(client)
//         .await
//         .expect("Failed to create Redis connection manager")
// }
