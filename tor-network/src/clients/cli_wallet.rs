// /home/inno/elights_jobes-research/tor-network/src/clients/cli_wallet.rs
use crate::error::TorNetworkError;
use crate::p2p_network::P2PCommand; // Import command type
use std::io::{self, Write};
use tokio::sync::mpsc; // For sending commands to the node

/// Starts an interactive CLI wallet. Requires a channel sender to communicate with a running P2P node.
pub async fn start_cli_wallet(
    mut p2p_command_sender: mpsc::Sender<P2PCommand>,
) -> Result<(), TorNetworkError> {
    println!("\n--- Elights Network CLI ---");
    println!("Type 'help' for available commands.");

    let mut stdin_reader = tokio::io::BufReader::new(tokio::io::stdin());
    let mut line_buffer = String::new();

    loop {
        print!("> ");
        io::stdout().flush().map_err(TorNetworkError::Io)?;
        line_buffer.clear();

        if stdin_reader.read_line(&mut line_buffer).await.map_err(TorNetworkError::Io)? == 0 {
             println!("Input stream closed. Exiting.");
             break; // End of input stream
        }

        let parts: Vec<&str> = line_buffer.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let cmd = parts[0].to_lowercase();
        let args = &parts[1..];

        let command_to_send: Option<P2PCommand> = match cmd.as_str() {
            "help" => {
                println!("Available commands:");
                println!("  subscribe <topic>     - Subscribe to a gossip topic");
                println!("  unsubscribe <topic>   - Unsubscribe from a gossip topic");
                println!("  publish <topic> <msg> - Publish a message to a gossip topic");
                println!("  findpeer <peer_id>    - Find addresses for a peer via Kademlia");
                println!("  getproviders <key>    - Find providers for a key via Kademlia");
                println!("  provide <key>         - Announce providing a key via Kademlia");
                println!("  exit                  - Exit the CLI");
                None // Help is handled locally
            }
            "subscribe" if args.len() == 1 => Some(P2PCommand::GossipsubSubscribe(args[0].to_string())),
            "unsubscribe" if args.len() == 1 => Some(P2PCommand::GossipsubUnsubscribe(args[0].to_string())),
            "publish" if args.len() >= 2 => {
                let topic = args[0].to_string();
                let data = args[1..].join(" ").as_bytes().to_vec();
                 Some(P2PCommand::GossipsubPublish { topic, data })
            }
            "findpeer" if args.len() == 1 => {
                match args[0].parse::<libp2p::PeerId>() {
                    Ok(peer_id) => Some(P2PCommand::KadFindPeer(peer_id)),
                    Err(_) => { println!("Error: Invalid Peer ID format."); None }
                }
            }
             "getproviders" if args.len() == 1 => Some(P2PCommand::KadGetProviders(args[0].to_string())),
             "provide" if args.len() == 1 => Some(P2PCommand::KadStartProviding(args[0].to_string())),
            "exit" => break, // Exit the loop
            _ => {
                println!("Error: Unknown command or invalid arguments. Type 'help'.");
                None
            }
        };

        // Send command to the P2P node if generated
        if let Some(command) = command_to_send {
             if let Err(e) = p2p_command_sender.send(command).await {
                 eprintln!("Error sending command to P2P node: {}. Exiting.", e);
                 break; // Exit if channel is broken
             }
        }
    }

    println!("CLI Wallet stopped.");
    Ok(())
}

use tokio::io::AsyncBufReadExt; // Required for read_line