// tor-network/p2p-network/libp2p.rs

// use libp2p::{identity, PeerId, Swarm, Multiaddr, NetworkBehaviour, development_transport};
use libp2p::{
    core::upgrade,
    identity,
    noise::{Keypair, NoiseConfig, X25519Spec},
    swarm::SwarmBuilder,
    tcp::TokioTcpConfig,
    yamux::YamuxConfig,
    Multiaddr, PeerId, Swarm, Transport,
};

use std::error::Error;

pub async fn build_swarm() -> Result<Swarm<impl libp2p::swarm::NetworkBehaviour>, Box<dyn Error>> {
    // Generate a keypair for authenticated encryption
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    // Set up an encrypted TCP transport over the Mplex and Yamux protocols
    let transport = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(Keypair::<X25519Spec>::new().into_authentic(&id_keys)?))
        .multiplex(YamuxConfig::default())
        .boxed();

    // Define your custom behaviour here
    let behaviour = MyBehaviour::new();

    // Build the Swarm
    let swarm = SwarmBuilder::new(transport, behaviour, peer_id)
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();

    Ok(swarm)
}

// #[derive(NetworkBehaviour)]
// struct MyBehaviour {
//     // Define your behaviour here
// }

// pub async fn start_p2p() {
//     let local_key = identity::Keypair::generate_ed25519();
//     let local_peer_id = PeerId::from(local_key.public());
//     println!("Local peer id: {:?}", local_peer_id);

//     let transport = development_transport(local_key.clone()).await.unwrap();
//     let behaviour = MyBehaviour {
//         // Initialize your behaviour
//     };
//     let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

//     // Start listening on a multiaddress
//     let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
//     swarm.listen_on(listen_addr).unwrap();
// }
