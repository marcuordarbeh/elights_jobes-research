// tor-network/p2p-network/onion_overlay.rs

use libp2p::{Multiaddr, PeerId, Swarm};
use std::error::Error;

pub async fn start_onion_overlay(swarm: &mut Swarm<impl libp2p::swarm::NetworkBehaviour>) -> Result<(), Box<dyn Error>> {
    // Listen on a Tor-compatible address
    let addr: Multiaddr = "/ip4/127.0.0.1/tcp/0".parse()?;
    swarm.listen_on(addr)?;

    // Event loop to handle incoming connections
    loop {
        match swarm.next().await {
            Some(event) => {
                // Handle the event
                println!("Swarm event: {:?}", event);
            }
            None => break,
        }
    }

    Ok(())
}

// // tor-network/p2p-network/noise.rs

// use libp2p::noise::{Keypair, NoiseConfig, X25519Spec, AuthenticKeypair, NoiseAuthenticated};

// pub fn build_noise_config() -> NoiseConfig<X25519Spec> {
//     let keypair = Keypair::<X25519Spec>::new().into_authentic(&identity::Keypair::generate_ed25519()).unwrap();
//     NoiseConfig::xx(keypair).into_authenticated()
// }
// ``
::contentReference[oaicite:0]{index=0}
 
