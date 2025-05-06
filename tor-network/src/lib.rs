// /home/inno/elights_jobes-research/tor-network/src/lib.rs

pub mod error;        // Error types
pub mod config;       // Configuration loading
pub mod p2p_network;  // Core libp2p logic (node, behaviour, transport)
pub mod clients;      // Client applications (e.g., CLI wallet)

// Re-export key components
pub use error::TorNetworkError;
pub use config::NodeConfig;
pub use p2p_network::{start_p2p_node, P2PEvent}; // Export node start function & events