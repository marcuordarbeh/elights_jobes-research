// /home/inno/elights_jobes-research/tor-network/src/p2p_network/mod.rs

mod behaviour;   // Define the NetworkBehaviour
mod transport;   // Build the transport layer
mod event_loop;  // Implement the main event loop logic
mod node;        // Node initialization and startup

pub use behaviour::{P2PNetworkBehaviour, P2PEvent, P2PCommand}; // Export behaviour, events, commands
pub use node::start_p2p_node; // Export the main node start function