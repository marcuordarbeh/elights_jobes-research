// /home/inno/elights_jobes-research/tor-network/src/p2p_network/behaviour.rs
use libp2p::{
    gossipsub::{self, IdentTopic as Topic, MessageAuthenticity, ValidationMode},
    identify::{self, Behaviour as IdentifyBehaviour, Config as IdentifyConfig},
    kad::{store::MemoryStore, Kademlia, KademliaConfig, KademliaEvent, QueryResult},
    ping::{Behaviour as PingBehaviour, Config as PingConfig, Event as PingEvent},
    swarm::NetworkBehaviour, // Use the derive macro
    PeerId,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use crate::error::TorNetworkError;

// Define the custom events that our behaviour will emit upwards.
#[derive(Debug)]
pub enum P2PEvent {
    Gossipsub(gossipsub::Event), // Events from Gossipsub (e.g., received message)
    Kademlia(KademliaEvent),    // Events from Kademlia DHT
    Identify(identify::Event),  // Events from Identify protocol
    Ping(PingEvent),            // Events from Ping protocol
    // Add custom events if needed
    // DiscoveredPeerViaTor(PeerId),
}

/// Commands that can be sent down to the behaviour layer.
#[derive(Debug, Clone)]
pub enum P2PCommand {
    GossipsubSubscribe(String),
    GossipsubUnsubscribe(String),
    GossipsubPublish { topic: String, data: Vec<u8> },
    KadFindPeer(PeerId),
    KadGetProviders(String), // Find providers for a key (e.g., content hash)
    KadStartProviding(String), // Announce providing a key
    // Add commands for other behaviours or custom logic
}


// Define the combined NetworkBehaviour struct using the derive macro.
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "P2PEvent")] // Specify the output event type
pub struct P2PNetworkBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub kademlia: Kademlia<MemoryStore>,
    pub identify: IdentifyBehaviour,
    pub ping: PingBehaviour,
    // TODO: Add RelayClient behaviour if using relays
    // pub relay_client: libp2p::relay::client::Behaviour,
}

impl P2PNetworkBehaviour {
    /// Creates a new P2PNetworkBehaviour instance.
    pub fn new(local_peer_id: PeerId /*, relay_transport: Option<libp2p::relay::client::Transport>*/) -> Result<Self, TorNetworkError> {
        // --- Kademlia Configuration ---
        let store = MemoryStore::new(local_peer_id);
        let kad_config = KademliaConfig::default();
        // kad_config.set_query_timeout(Duration::from_secs(60)); // Example customization
        let kademlia = Kademlia::with_config(local_peer_id, store, kad_config);

        // --- Gossipsub Configuration ---
        // Use a hashing function for message IDs (prevents duplicates)
        let message_id_fn = |message: &gossipsub::Message| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            gossipsub::MessageId::from(s.finish().to_string())
        };
        // Build Gossipsub config
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10)) // Recommended
            .validation_mode(ValidationMode::Strict) // Message validation
            .message_id_fn(message_id_fn) // Use custom message ID fn
            // TODO: Configure message limits, fanout TTLs etc. as needed
            .build()
            .map_err(|e| TorNetworkError::Internal(format!("Failed to build Gossipsub config: {}", e)))?;
        // Build the Gossipsub behaviour
        let gossipsub = gossipsub::Behaviour::new(
            MessageAuthenticity::Signed(libp2p::identity::Keypair::generate_ed25519()), // Use generated key for signing messages
            gossipsub_config,
        )?; // Error handled by From trait for GossipsubError

        // --- Identify Configuration ---
        let identify_config = IdentifyConfig::new(
            "/elights-p2p/1.0.0".to_string(), // Protocol version string
            libp2p::identity::Keypair::generate_ed25519().public(), // Send public key info
        )
        .with_agent_version(format!("elights-node/{}", env!("CARGO_PKG_VERSION")));
        let identify = IdentifyBehaviour::new(identify_config);

        // --- Ping Configuration ---
        let ping_config = PingConfig::new().with_keep_alive(true);
        let ping = PingBehaviour::new(ping_config);

        // --- Relay Client Configuration ---
        // let relay_client = match relay_transport {
        //      Some(transport) => libp2p::relay::client::Behaviour::new(local_peer_id, transport),
        //      None => // Handle case where relay is not enabled/needed
        // }

        Ok(Self {
            gossipsub,
            kademlia,
            identify,
            ping,
            // relay_client,
        })
    }
}

// Implement From traits to map sub-behaviour events to the main P2PEvent enum
impl From<gossipsub::Event> for P2PEvent { fn from(event: gossipsub::Event) -> Self { P2PEvent::Gossipsub(event) } }
impl From<KademliaEvent> for P2PEvent { fn from(event: KademliaEvent) -> Self { P2PEvent::Kademlia(event) } }
impl From<identify::Event> for P2PEvent { fn from(event: identify::Event) -> Self { P2PEvent::Identify(event) } }
impl From<PingEvent> for P2PEvent { fn from(event: PingEvent) -> Self { P2PEvent::Ping(event) } }
// impl From<libp2p::relay::client::Event> for P2PEvent { fn from(event: libp2p::relay::client::Event) -> Self { ... } }

// Implement conversion from Gossipsub error to allow `?` operator in constructor
impl From<String> for TorNetworkError {
     fn from(err: String) -> Self {
         TorNetworkError::Internal(err)
     }
 }