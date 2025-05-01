// /home/inno/elights_jobes-research/tor-network/src/p2p_network/noise.rs
use libp2p::noise::{AuthenticKeypair, Keypair, NoiseAuthenticated, NoiseConfig, X25519Spec};
use libp2p::identity::Keypair as IdentityKeypair;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NoiseError {
    #[error("Failed to generate authentic Noise keypair: {0}")]
    KeypairGenerationError(String),
}

/// Builds a Noise XX handshake configuration using Ed25519 identity keys.
pub fn build_noise_config(
    id_keys: &IdentityKeypair,
) -> Result<NoiseAuthenticated<X25519Spec, noise::X25519, ()>, NoiseError> {
    let noise_static_dh_keys = Keypair::<X25519Spec>::new();
    let noise_authentic_keys: AuthenticKeypair<X25519Spec> = noise_static_dh_keys
        .into_authentic(id_keys)
        .map_err(|e| NoiseError::KeypairGenerationError(e.to_string()))?;

    Ok(NoiseConfig::xx(noise_authentic_keys).into_authenticated())
}