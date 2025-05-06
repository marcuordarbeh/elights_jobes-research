// /home/inno/elights_jobes-research/tor-network/src/p2p_network/node.rs
use crate::config::NodeConfig;
use crate::error::TorNetworkError;
use crate::p2p_network::{
    behaviour::P2PNetworkBehaviour,
    event_loop::run_p2p_event_loop,
    transport::build_p2p_transport,
    P2PCommand, P2PEvent, // Import command/event types
};
use libp2p::{identity, Multiaddr, PeerId, Swarm, SwarmBuilder};
use tokio::sync::mpsc; // For command channel

/// Starts the main P2P node.
/// Returns a channel sender to send commands to the node's event loop.
pub async fn start_p2p_node(
    config: NodeConfig,
) -> Result<mpsc::Sender<P2PCommand>, TorNetworkError> {
    log::info!("Starting Elights P2P Node...");

    // 1. Generate or Load Identity Keys
    // TODO: Persist and load keys instead of generating each time for stable PeerId
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    log::info!("Local Peer ID: {}", local_peer_id);

    // 2. Build the Transport Stack
    let transport = build_p2p_transport(&local_key, &config).await?;

    // 3. Build the Network Behaviour
    let behaviour = P2PNetworkBehaviour::new(local_peer_id)?;

    // 4. Create the Swarm
    let mut swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, local_peer_id).build();

    // 5. Configure Listen Address
    let listen_addr: Multiaddr = config.listen_address.parse()
        .map_err(|e| TorNetworkError::Configuration(format!("Invalid listen address '{}': {}", config.listen_address, e)))?;
    swarm.listen_on(listen_addr.clone())?;
    log::info!("Node listening on: {}", listen_addr);

    // Log all active listeners
    // Need to wait briefly for listeners to potentially bind before logging
    // This might be better logged within the event loop on NewListenAddr event
    // for listener in swarm.listeners() {
    //     log::info!("Effective listening address: {}", listener);
    // }

    // 6. Add Bootstrap Peers to Kademlia routing table
    for peer_addr in &config.bootstrap_peers {
        log::info!("Adding initial bootstrap peer: {}", peer_addr);
        // Extract PeerId if present in the multiaddr, otherwise Kademlia handles it via Identify
        if let Some(peer_id) = peer_addr.iter().find_map(|p| match p {
            libp2p::multiaddr::Protocol::P2p(hash) => PeerId::from_multihash(hash).ok(),
            _ => None,
        }) {
            swarm.behaviour_mut().kademlia.add_address(&peer_id, peer_addr.clone());
        } else {
             log::warn!("Bootstrap peer address {} does not contain a PeerId, Kademlia will try to discover it.", peer_addr);
             // Kademlia might need an initial bootstrap *request* if address has no PeerId
             // swarm.behaviour_mut().kademlia.bootstrap().ok(); // Start initial bootstrap
        }
    }
     // Initiate a Kademlia bootstrap process if peers were added
     if !config.bootstrap_peers.is_empty() {
          match swarm.behaviour_mut().kademlia.bootstrap() {
             Ok(_) => log::info!("Kademlia bootstrap process initiated."),
             Err(e) => log::warn!("Failed to initiate Kademlia bootstrap: {:?}", e),
          }
     } else {
          log::warn!("No bootstrap peers configured. Peer discovery might be slow.");
     }


    // 7. Create Command Channel
    // Buffer size can be adjusted based on expected command throughput
    let (command_sender, command_receiver) = mpsc::channel::<P2PCommand>(32);

    // 8. Spawn the Event Loop in a separate task
    tokio::spawn(run_p2p_event_loop(swarm, command_receiver, config));

    log::info!("P2P Node startup sequence complete. Event loop running.");

    // Return the sender half of the command channel to the caller
    Ok(command_sender)
}