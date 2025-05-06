// /home/inno/elights_jobes-research/tor-network/src/p2p_network/event_loop.rs
use crate::config::NodeConfig;
use crate::error::TorNetworkError;
use crate::p2p_network::{P2PNetworkBehaviour, P2PCommand, P2PEvent};
use futures::StreamExt;
use libp2p::{
    gossipsub::{self, IdentTopic as Topic, MessageId}, // Use IdentTopic
    kad::{QueryId, QueryResult, GetProvidersOk, GetProvidersError},
    swarm::{Swarm, SwarmEvent},
    PeerId,
};
use tokio::sync::mpsc;

/// Runs the main event loop, processing incoming events from the Swarm and commands from the application.
pub(super) async fn run_p2p_event_loop(
    mut swarm: Swarm<P2PNetworkBehaviour>,
    mut command_receiver: mpsc::Receiver<P2PCommand>,
    _config: NodeConfig, // Keep config if needed for loop logic
) -> Result<(), TorNetworkError> { // Return type indicates loop termination error
    log::info!("P2P Event Loop started.");

    loop {
        tokio::select! {
            // Process commands received from the application/other parts of the node
            Some(command) = command_receiver.recv() => {
                if let Err(e) = handle_command(&mut swarm, command).await {
                     log::error!("Failed to handle command: {}", e);
                     // Decide if error is fatal or recoverable
                }
            },
            // Process events originating from the libp2p Swarm
            event = swarm.select_next_some() => {
                handle_swarm_event(&mut swarm, event).await;
            },
            // Add other branches for signals (e.g., shutdown) if needed
            // _ = tokio::signal::ctrl_c() => {
            //     log::info!("Shutdown signal received, terminating event loop.");
            //     break;
            // }
        }
    }
    // Ok(()) // Loop normally doesn't exit unless shutdown implemented
}

/// Handles commands sent to the P2P network behaviour.
async fn handle_command(
    swarm: &mut Swarm<P2PNetworkBehaviour>,
    command: P2PCommand,
) -> Result<(), TorNetworkError> {
    log::debug!("Received P2P Command: {:?}", command);
    match command {
        P2PCommand::GossipsubSubscribe(topic_name) => {
            let topic = Topic::new(topic_name);
            if let Err(e) = swarm.behaviour_mut().gossipsub.subscribe(&topic) {
                 log::error!("Failed to subscribe to Gossipsub topic '{}': {:?}", topic, e);
                 return Err(TorNetworkError::GossipsubSubscription(e));
            } else {
                 log::info!("Subscribed to Gossipsub topic: {}", topic);
            }
        }
        P2PCommand::GossipsubUnsubscribe(topic_name) => {
             let topic = Topic::new(topic_name);
             if let Err(e) = swarm.behaviour_mut().gossipsub.unsubscribe(&topic) {
                 log::error!("Failed to unsubscribe from Gossipsub topic '{}': {:?}", topic, e);
                 // Decide how to handle unsubscribe errors
             } else {
                  log::info!("Unsubscribed from Gossipsub topic: {}", topic);
             }
        }
        P2PCommand::GossipsubPublish { topic: topic_name, data } => {
            let topic = Topic::new(topic_name);
             // Ensure subscribed to the topic first? Gossipsub might handle this.
             match swarm.behaviour_mut().gossipsub.publish(topic.clone(), data) {
                 Ok(message_id) => log::info!("Published message to Gossipsub topic '{}', ID: {}", topic, message_id),
                 Err(e) => {
                     log::error!("Failed to publish to Gossipsub topic '{}': {:?}", topic, e);
                     return Err(TorNetworkError::GossipsubPublish(e));
                 }
             }
        }
        P2PCommand::KadFindPeer(peer_id) => {
             log::info!("Kademlia: Searching for peer {}", peer_id);
             swarm.behaviour_mut().kademlia.get_closest_peers(peer_id);
             // Result will come back as a Kademlia event
        }
        P2PCommand::KadGetProviders(key_str) => {
            log::info!("Kademlia: Getting providers for key '{}'", key_str);
            let key = libp2p::kad::RecordKey::new(&key_str.as_bytes()); // Example key conversion
            swarm.behaviour_mut().kademlia.get_providers(key);
             // Result will come back as a Kademlia event
        }
         P2PCommand::KadStartProviding(key_str) => {
             log::info!("Kademlia: Announcing providing key '{}'", key_str);
             let key = libp2p::kad::RecordKey::new(&key_str.as_bytes());
             match swarm.behaviour_mut().kademlia.start_providing(key) {
                  Ok(query_id) => log::debug!("Kademlia: Start providing query initiated: {:?}", query_id),
                  Err(e) => log::error!("Kademlia: Failed to start providing: {:?}", e),
             }
         }
         // Handle other commands
    }
    Ok(())
}


/// Handles events emitted by the libp2p Swarm.
async fn handle_swarm_event(
    swarm: &mut Swarm<P2PNetworkBehaviour>,
    event: SwarmEvent<P2PEvent, impl std::error::Error>, // Use generic connection error type
) {
     match event {
         // --- Connection Management Events ---
         SwarmEvent::NewListenAddr { address, .. } => {
             log::info!("Node listening on new address: {}", address);
         }
         SwarmEvent::ListenerClosed { addresses, reason, .. } => {
              log::warn!("Listener closed. Addresses: {:?}, Reason: {:?}", addresses, reason);
         }
          SwarmEvent::ListenerError { error, .. } => {
              log::error!("Listener error: {}", error);
         }
         SwarmEvent::IncomingConnection { local_addr, send_back_addr } => {
             log::debug!("Incoming connection: local={}, remote={}", local_addr, send_back_addr);
         }
         SwarmEvent::ConnectionEstablished { peer_id, endpoint, num_established, .. } => {
             log::info!(
                 "Connection established with peer: {}. Endpoint: {}. Total connections: {}",
                 peer_id, endpoint.get_remote_address(), num_established
             );
             // Add peer to Kademlia table if not already present
             swarm.behaviour_mut().kademlia.add_address(&peer_id, endpoint.get_remote_address().clone());
         }
         SwarmEvent::ConnectionClosed { peer_id, cause, num_established, .. } => {
             log::info!(
                 "Connection closed with peer: {}. Cause: {:?}. Total connections: {}",
                 peer_id, cause, num_established
             );
             // Remove peer from Kademlia table? Or let Kademlia handle eviction.
             // swarm.behaviour_mut().kademlia.remove_peer(&peer_id);
         }
         SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
              log::warn!("Failed to dial peer {:?}: {}", peer_id, error);
         }
          SwarmEvent::IncomingConnectionError { local_addr, send_back_addr, error, .. } => {
               log::warn!("Failed incoming connection: local={}, remote={}, error={}", local_addr, send_back_addr, error);
           }
         SwarmEvent::ExpiredListenAddr { address, .. } => {
             log::warn!("Expired listening address: {}", address);
         }
         SwarmEvent::Dialing(peer_id) => {
             log::debug!("Dialing peer: {}", peer_id);
         }

         // --- Behaviour-Specific Events ---
         SwarmEvent::Behaviour(p2p_event) => {
             log::debug!("Received Behaviour Event: {:?}", p2p_event);
             match p2p_event {
                 // --- Gossipsub Events ---
                 P2PEvent::Gossipsub(gossipsub::Event::Message {
                     propagation_source: _peer_id,
                     message_id,
                     message,
                 }) => {
                     log::info!(
                         "Gossipsub: Received message: '{}' from topic '{}', ID: {}",
                         String::from_utf8_lossy(&message.data),
                         message.topic,
                         message_id
                     );
                     // TODO: Process received gossip message data
                 }
                 P2PEvent::Gossipsub(gossipsub::Event::Subscribed { peer_id, topic }) => {
                      log::info!("Gossipsub: Peer {} subscribed to topic '{}'", peer_id, topic);
                 }
                 P2PEvent::Gossipsub(gossipsub::Event::Unsubscribed { peer_id, topic }) => {
                      log::info!("Gossipsub: Peer {} unsubscribed from topic '{}'", peer_id, topic);
                 }
                 P2PEvent::Gossipsub(gossipsub::Event::GossipsubNotSupported { peer_id }) => {
                      log::warn!("Gossipsub: Peer {} does not support gossipsub", peer_id);
                 }

                 // --- Kademlia Events ---
                 P2PEvent::Kademlia(KademliaEvent::OutboundQueryProgressed { id, result, stats, step }) => {
                      log::debug!("Kademlia query {:?} progressed. Step: {:?}, Stats: {:?}", id, step, stats);
                      match result {
                           QueryResult::GetClosestPeers(Ok(ok)) => {
                                if ok.peers.is_empty() {
                                    log::warn!("Kademlia GetClosestPeers query {:?} finished with no peers.", id);
                                } else {
                                     log::info!("Kademlia GetClosestPeers query {:?} found peers: {:?}", id, ok.peers);
                                }
                            }
                            QueryResult::GetClosestPeers(Err(err)) => {
                                 log::error!("Kademlia GetClosestPeers query {:?} failed: {:?}", id, err);
                            }
                            QueryResult::GetProviders(Ok(GetProvidersOk::FoundProviders { key, providers, .. })) => {
                                 log::info!("Kademlia GetProviders query {:?} found providers for key {:?}: {:?}", id, String::from_utf8_lossy(key.as_ref()), providers);
                            }
                            QueryResult::GetProviders(Ok(GetProvidersOk::FinishedWithNoAdditionalRecord { .. })) => {
                                log::debug!("Kademlia GetProviders query {:?} finished with no new records.", id);
                            }
                            QueryResult::GetProviders(Err(err @ GetProvidersError::Timeout { .. })) => {
                                 log::warn!("Kademlia GetProviders query {:?} timed out: {:?}", id, err);
                            }
                            QueryResult::StartProviding(Ok(ok)) => {
                                 log::info!("Kademlia StartProviding query {:?} finished successfully for key {:?}", id, ok.key);
                             }
                            QueryResult::StartProviding(Err(err)) => {
                                 log::error!("Kademlia StartProviding query {:?} failed: {:?}", id, err);
                             }
                            // Handle other Kademlia query results (GetRecord, PutRecord) if used
                           _ => {} // Ignore other query results for now
                      }
                 }
                 P2PEvent::Kademlia(KademliaEvent::RoutingUpdated { peer, is_new, addresses, .. }) => {
                      log::debug!("Kademlia routing table updated. Peer: {}, New: {}, Addresses: {:?}", peer, is_new, addresses);
                 }
                  P2PEvent::Kademlia(other) => {
                       log::trace!("Unhandled Kademlia event: {:?}", other);
                  }

                 // --- Identify Events ---
                 P2PEvent::Identify(identify::Event::Received { peer_id, info }) => {
                     log::info!("Identify: Received info from peer: {}", peer_id);
                     log::debug!("  -> Protocol Version: {}", info.protocol_version);
                     log::debug!("  -> Agent Version: {}", info.agent_version);
                     log::debug!("  -> Observed Address: {}", info.observed_addr);
                     log::debug!("  -> Listen Addresses: {:?}", info.listen_addrs);
                     log::debug!("  -> Supported Protocols: {:?}", info.protocols);
                      // Add identified peer addresses to Kademlia
                      for addr in info.listen_addrs {
                           swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
                      }
                 }
                 P2PEvent::Identify(identify::Event::Sent { peer_id }) => {
                     log::trace!("Identify: Sent info to peer: {}", peer_id);
                 }
                 P2PEvent::Identify(identify::Event::Pushed { peer_id, .. }) => {
                      log::trace!("Identify: Pushed info to peer: {}", peer_id);
                  }
                 P2PEvent::Identify(identify::Event::Error { peer_id, error }) => {
                      log::warn!("Identify: Error with peer {}: {}", peer_id, error);
                 }

                 // --- Ping Events ---
                  P2PEvent::Ping(ping_event) => {
                     match ping_event.result {
                          Ok(rtt) => log::trace!("Ping: RTT to peer {} is {}ms", ping_event.peer, rtt.as_millis()),
                          Err(e) => log::warn!("Ping: Failed to ping peer {}: {}", ping_event.peer, e),
                     }
                  }
             }
         }
         // Handle other swarm events if needed
         other => {
             log::trace!("Unhandled Swarm event: {:?}", other);
         }
     }
}