// /home/inno/elights_jobes-research/tor-network/src/p2p_network/transport.rs
use crate::config::NodeConfig;
use crate::error::TorNetworkError;
use libp2p::{
    core::muxing::StreamMuxerBox,
    core::transport::{upgrade::Version, Boxed},
    dns::TokioDnsConfig,
    identity, noise, tcp, yamux, PeerId, Transport,
};
use std::io;
use std::time::Duration;

// Type alias for the boxed transport
pub type P2PTransport = Boxed<(PeerId, StreamMuxerBox)>;

/// Builds the libp2p transport stack.
/// Supports TCP directly and potentially via a SOCKS5 proxy (Tor).
pub async fn build_p2p_transport(
    local_key: &identity::Keypair,
    config: &NodeConfig,
) -> Result<P2PTransport, TorNetworkError> {
    // --- Base Transport (TCP + DNS) ---
    let tcp_transport = tcp::tokio::Transport::new(tcp::Config::default().nodelay(true));
    let transport = TokioDnsConfig::system(tcp_transport)
        .map_err(|e| TorNetworkError::Transport(format!("DNS config error: {}", e)))?;

    // --- Optional SOCKS5 Proxy (Tor) ---
    // TODO: Integrate SOCKS5 proxy transport if configured.
    // This requires a SOCKS5 compatible transport implementation for libp2p.
    // Example using a hypothetical `Socks5Transport` crate:
    let transport = if let Some(proxy_addr) = &config.tor_socks_proxy {
         log::info!("Configuring transport to use SOCKS5 proxy: {}", proxy_addr);
         // Placeholder: Replace with actual SOCKS5 transport setup
         // let proxy = Socks5Transport::new(transport, proxy_addr).await?;
         // return Ok(proxy.boxed()); // Assuming Socks5Transport implements Transport correctly
         log::warn!("SOCKS5 proxy transport is configured but not implemented yet. Using direct TCP.");
         transport.boxed() // Fallback to direct TCP if SOCKS not implemented
    } else {
        transport.boxed() // Use direct TCP transport
    };


    // --- Security Layer (Noise XX) ---
    let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(local_key)
        .map_err(|e| TorNetworkError::Noise(format!("Noise key generation failed: {}", e)))?;
    let noise_config = noise::NoiseConfig::xx(noise_keys).into_authenticated();

    // --- Multiplexing Layer (Yamux) ---
    let yamux_config = yamux::Config::default();

    // --- Build the final authenticated and multiplexed transport ---
    let final_transport = transport
        .upgrade(Version::V1Lazy) // Use lazy upgrades for efficiency
        .authenticate(noise_config)
        .multiplex(yamux_config)
        .timeout(config.dial_timeout) // Apply dial timeout
        .map(|(peer_id, muxer), _| (peer_id, StreamMuxerBox::new(muxer))) // Box the muxer
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e)) // Map timeout error
        .boxed(); // Box the final transport

    Ok(final_transport)
}