// /home/inno/elights_jobes-research/tor-network/src/clients/node.rs
// This node acts as a peer in the libp2p network, potentially routing via Tor.

use crate::p2p_network::libp2p::build_swarm;
use crate::p2p_network::onion_overlay::run_p2p_event_loop;
use crate::error::TorNetworkError;
use futures::StreamExt; // Required for swarm.next().await

/// Starts the Elights P2P node.
pub async fn start_node() -> Result<(), TorNetworkError> {
    println!("Starting Elights P2P Node...");

    // Build the libp2p swarm with Noise encryption and Yamux multiplexing
    let mut swarm = build_swarm().await?;
    println!("Swarm built. Local Peer ID: {}", swarm.local_peer_id());

    // Listen on a local TCP address. Tor routing would typically be configured
    // at the system level or via specific libp2p transports if available,
    // or by dialing peers through a Tor SOCKS proxy.
    // This example listens locally; dialing specific Tor peers requires more setup.
    let listen_addr = "/ip4/0.0.0.0/tcp/0"
        .parse()
        .map_err(|e| TorNetworkError::P2pInit(format!("Invalid listen address: {}", e)))?;

    swarm.listen_on(listen_addr.clone())
         .map_err(|e| TorNetworkError::ListenError(format!("Failed to listen on {}: {}", listen_addr, e)))?;

    println!("Node listening on addresses: {:?}", swarm.listeners().collect::<Vec<_>>());

    // Run the main event loop to handle network events
    run_p2p_event_loop(swarm).await?;

    println!("Elights P2P Node stopped.");
    Ok(())
}