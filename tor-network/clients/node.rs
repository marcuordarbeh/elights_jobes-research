// tor-network/clients/node.rs

use libp2p::Swarm;
use std::error::Error;

pub async fn start_node() -> Result<(), Box<dyn Error>> {
    let mut swarm = build_swarm().await?;

    // Start the onion overlay network
    start_onion_overlay(&mut swarm).await?;

    Ok(())
}
