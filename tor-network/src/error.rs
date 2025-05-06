// /home/inno/elights_jobes-research/tor-network/src/error.rs
use thiserror::Error;

/// Errors related to the Tor/P2P network component.
#[derive(Error, Debug)]
pub enum TorNetworkError {
    #[error("Configuration Error: {0}")]
    Configuration(String),

    #[error("P2P Network Initialization Failed: {0}")]
    P2pInit(String),

    #[error("Transport Error: {0}")]
    Transport(String),

    #[error("Listen Error: {0}")]
    Listen(#[from] libp2p::TransportError<std::io::Error>),

    #[error("Dial Error: {0}")]
    Dial(#[from] libp2p::swarm::DialError),

    #[error("Identity Key Error: {0}")]
    IdentityKey(String),

    #[error("Noise Protocol Error: {0}")]
    Noise(String), // Wrap specific Noise errors if needed

    #[error("Input/Output Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CLI Wallet Error: {0}")]
    CliWallet(String),

    #[error("Gossipsub Subscription Error: {0}")]
    GossipsubSubscription(#[from] libp2p::gossipsub::SubscriptionError),

    #[error("Gossipsub Publish Error: {0}")]
    GossipsubPublish(#[from] libp2p::gossipsub::PublishError),

    #[error("Kademlia Query Error: {0}")]
    KademliaQuery(String), // Wrap specific Kademlia errors if needed

    #[error("Internal Network Error: {0}")]
    Internal(String),
}

// Helper for converting identity errors
impl From<libp2p::identity::error::DecodingError> for TorNetworkError {
    fn from(err: libp2p::identity::error::DecodingError) -> Self {
        TorNetworkError::IdentityKey(format!("Identity key decoding error: {}", err))
    }
}
impl From<libp2p::identity::error::SigningError> for TorNetworkError {
     fn from(err: libp2p::identity::error::SigningError) -> Self {
        TorNetworkError::IdentityKey(format!("Identity key signing error: {}", err))
    }
}