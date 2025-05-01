// /home/inno/elights_jobes-research/tor-network/src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TorNetworkError {
    #[error("P2P Network initialization failed: {0}")]
    P2pInit(String),

    #[error("Failed to listen on address: {0}")]
    ListenError(String),

    #[error("Noise configuration error: {0}")]
    NoiseConfig(#[from] crate::p2p_network::noise::NoiseError), // Example From impl

    #[error("Transport error: {0}")]
    Transport(String), // Wrap libp2p transport errors

    #[error("Dial error: {0}")]
    DialError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CLI Wallet error: {0}")]
    CliWallet(String),

    #[error("Onion Service setup error: {0}")]
    OnionService(String),
}

// Example conversion from libp2p::TransportError if needed
// impl<T: std::error::Error + Send + Sync + 'static> From<libp2p::TransportError<T>> for TorNetworkError {
//    fn from(err: libp2p::TransportError<T>) -> Self {
//        TorNetworkError::Transport(err.to_string())
//    }
//}