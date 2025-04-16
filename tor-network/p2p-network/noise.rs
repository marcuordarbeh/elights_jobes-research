// tor-network/p2p-network/noise.rs

use libp2p::noise::{Keypair, NoiseConfig, X25519Spec};
use libp2p::identity::Keypair as IdentityKeypair;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NoiseError {
    #[error("Failed to generate authentic keypair: {0}")]
    KeypairGenerationError(String),
}

pub fn build_noise_config(id_keys: &IdentityKeypair) -> Result<NoiseConfig<X25519Spec>, NoiseError> {
    let noise_keys = Keypair::<X25519Spec>::new()
        .into_authentic(id_keys)
        .map_err(|e| NoiseError::KeypairGenerationError(e.to_string()))?;
    Ok(NoiseConfig::xx(noise_keys))
}
