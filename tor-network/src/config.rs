// /home/inno/elights_jobes-research/tor-network/src/config.rs
use crate::error::TorNetworkError;
use std::{env, net::SocketAddr, str::FromStr, time::Duration};

/// Configuration for the P2P Node.
#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub listen_address: String,     // Multiaddr format, e.g., "/ip4/0.0.0.0/tcp/0"
    pub bootstrap_peers: Vec<libp2p::Multiaddr>,
    pub tor_socks_proxy: Option<String>, // e.g., "127.0.0.1:9050"
    pub dial_timeout: Duration,
    pub idle_connection_timeout: Duration,
    // Add Gossipsub/Kademlia specific configs if needed
}

impl NodeConfig {
    /// Loads node configuration from environment variables.
    pub fn from_env() -> Result<Self, TorNetworkError> {
        dotenv::dotenv().ok(); // Load .env file if feature enabled

        let listen_address = env::var("P2P_LISTEN_ADDR")
            .unwrap_or_else(|_| "/ip4/0.0.0.0/tcp/0".to_string());

        let bootstrap_peers_str = env::var("P2P_BOOTSTRAP_PEERS").unwrap_or_default();
        let bootstrap_peers = bootstrap_peers_str
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        let tor_socks_proxy = env::var("TOR_SOCKS_PROXY").ok() // Use var set by docker-compose or locally
            .filter(|s| !s.is_empty());

        let dial_timeout_secs = env::var("P2P_DIAL_TIMEOUT_SECS")
            .unwrap_or_else(|_| "20".to_string())
            .parse::<u64>()
            .map_err(|_| TorNetworkError::Configuration("Invalid P2P_DIAL_TIMEOUT_SECS".to_string()))?;

         let idle_timeout_secs = env::var("P2P_IDLE_TIMEOUT_SECS")
            .unwrap_or_else(|_| "60".to_string())
            .parse::<u64>()
            .map_err(|_| TorNetworkError::Configuration("Invalid P2P_IDLE_TIMEOUT_SECS".to_string()))?;


        log::info!("Node Config Loaded:");
        log::info!("  - Listen Address: {}", listen_address);
        log::info!("  - Bootstrap Peers: {:?}", bootstrap_peers);
        log::info!("  - Tor SOCKS Proxy: {:?}", tor_socks_proxy);
        log::info!("  - Dial Timeout: {}s", dial_timeout_secs);
        log::info!("  - Idle Timeout: {}s", idle_timeout_secs);

        Ok(Self {
            listen_address,
            bootstrap_peers,
            tor_socks_proxy,
            dial_timeout: Duration::from_secs(dial_timeout_secs),
            idle_connection_timeout: Duration::from_secs(idle_timeout_secs),
        })
    }
}