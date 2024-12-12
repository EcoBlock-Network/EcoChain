use std::{env, error::Error};
use std::time::Duration;
use futures::StreamExt;
use libp2p::{
    ping,
    identify,
    mdns,
    rendezvous::{client, server, Namespace},
    tcp::Config as TcpConfig,
    swarm::{NetworkBehaviour, SwarmEvent},
};
use tracing_subscriber::EnvFilter;
use libp2p_identity::Keypair;
use std::fs;
use std::io::Write;



#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent")]
struct MyBehaviour {
    identify: identify::Behaviour,
    rendezvous_client: client::Behaviour,
    rendezvous_server: server::Behaviour,
    ping: ping::Behaviour,
    mdns: mdns::async_io::Behaviour,
}

#[derive(Debug)]
enum MyBehaviourEvent {
    Identify(identify::Event),
    RendezvousClient(client::Event),
    #[allow(dead_code)]
    RendezvousServer(server::Event),
    Ping(ping::Event),
    Mdns(mdns::Event),
}

impl From<identify::Event> for MyBehaviourEvent {
    fn from(event: identify::Event) -> Self {
        MyBehaviourEvent::Identify(event)
    }
}

impl From<client::Event> for MyBehaviourEvent {
    fn from(event: client::Event) -> Self {
        MyBehaviourEvent::RendezvousClient(event)
    }
}

impl From<server::Event> for MyBehaviourEvent {
    fn from(event: server::Event) -> Self {
        MyBehaviourEvent::RendezvousServer(event)
    }
}

impl From<ping::Event> for MyBehaviourEvent {
    fn from(event: ping::Event) -> Self {
        MyBehaviourEvent::Ping(event)
    }
}

impl From<mdns::Event> for MyBehaviourEvent {
    fn from(event: mdns::Event) -> Self {
        MyBehaviourEvent::Mdns(event)
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let is_server = args.iter().any(|arg| arg == "--server");
    let is_client = args.iter().any(|arg| arg == "--client");

    println!(
        "Mode d'exécution : {}",
        if is_server {
            "Serveur"
        } else if is_client {
            "Client"
        } else {
            "Client et Serveur (par défaut)"
        }
    );

    // INIT
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
        .unwrap();
    
        fn generate_or_load_keypair() -> Keypair {
            let key_path = "peer_keypair";
        
            if let Ok(bytes) = fs::read(key_path) {
                match Keypair::from_protobuf_encoding(&bytes) {
                    Ok(keypair) => {
                        println!("Keys loaded from {key_path}");
                        keypair
                    }
                    Err(_) => {
                        panic!("Error loading keys from {key_path}");
                    }
                }
            } else {
                let keypair = Keypair::generate_ed25519();
                let encoded_key = keypair
                    .to_protobuf_encoding()
                    .expect("Error encoding keys to protobuf.");
        
                let mut file = fs::File::create(key_path).expect("Error creating keys file.");
                file.write_all(&encoded_key)
                    .expect("Error writing keys to file.");
                println!("New keys generated and saved to {key_path}");
                keypair
            }
        }
        
    // GENERATE PEER ID
    let local_key = generate_or_load_keypair();
    let local_peer_id = libp2p::PeerId::from(local_key.public());
    println!("Local Peer ID: {}", local_peer_id);

    // BUILD THE SWARM
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_async_std()
        .with_tcp(
            TcpConfig::default(),
            libp2p::tls::Config::new,
            || libp2p::yamux::Config::default(),
        )?
        .with_behaviour(|_| MyBehaviour {
            identify: identify::Behaviour::new(identify::Config::new(
                "ecoCore-discovery/1.0.0".to_string(),
                local_key.public(),
            )),
            rendezvous_client: if is_client || (!is_client && !is_server) {
                client::Behaviour::new(local_key.clone())
            } else {
                client::Behaviour::new(local_key.clone())
            },
            rendezvous_server: if is_server || (!is_client && !is_server) {
                server::Behaviour::new(server::Config::default())
            } else {
                server::Behaviour::new(server::Config::default())
            },
            ping: ping::Behaviour::new(ping::Config::new().with_interval(Duration::from_secs(1))),
            mdns: mdns::async_io::Behaviour::new(mdns::Config::default(), local_peer_id).unwrap(),
        })?
        .build();

    // LISTEN ON ALL INTERFACES (Serveur uniquement)
    if is_server || (!is_client && !is_server) {
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        println!("Swarm listening on /ip4/0.0.0.0/tcp/0");
    }

    // REGISTER IN RENDEZVOUS SERVER (Client uniquement)
    if is_client || (!is_client && !is_server) {
        let namespace = Namespace::new("ecoCore".to_string()).expect("Invalid namespace");
        let _ = swarm.behaviour_mut().rendezvous_client.register(namespace.clone(), local_peer_id, Some(3600));
        println!("Registered in namespace: {}", namespace);
    }

    let mut connected_peers: Vec<libp2p::PeerId> = Vec::new();

    // SWARM EVENT LOOP
    loop {
        println!("Waiting for SwarmEvent...");
    
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on: {address}");
            }
            // SwarmEvent::ConnectionEstablished { peer_id, .. } => {
            //     println!("Connected to peer: {peer_id}");
            //     if !connected_peers.contains(&peer_id) {
            //         connected_peers.push(peer_id);
            //     }
            // }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                println!("Connected to peer: {peer_id}");
                if !connected_peers.contains(&peer_id) {
                    connected_peers.push(peer_id);
                }
            
                // Interagir après la connexion
                if is_client {
                    let message = "Hello from client!";
                    println!("Client sending message to {peer_id}: {message}");
                    // Simulez un envoi de message ici (remplacez par une vraie logique si nécessaire)
                }
            
                if is_server {
                    let response = "Hello from server!";
                    println!("Server responding to {peer_id}: {response}");
                    // Simulez une réponse ici (remplacez par une vraie logique si nécessaire)
                }
            }
            SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(peers))) => {
                for (peer_id, addr) in peers {
                    if peer_id != local_peer_id && !connected_peers.contains(&peer_id) {
                        println!("Discovered peer via mDNS: {} at {:?}", peer_id, addr);
                        if let Err(err) = swarm.dial(addr.clone()) {
                            println!("Failed to dial discovered peer {}: {:?}", peer_id, err);
                        } else {
                            println!("Dialing discovered peer at: {:?}", addr);
                        }
                    }
                }
            }
            SwarmEvent::Behaviour(MyBehaviourEvent::RendezvousClient(client::Event::Discovered { registrations, .. })) => {
                for reg in registrations {
                    println!(
                        "Discovered peer: {} in namespace: {}",
                        reg.record.peer_id(),
                        reg.namespace
                    );
                    if let Some(addr) = reg.record.addresses().first() {
                        if let Err(err) = swarm.dial(addr.clone()) {
                            println!("Failed to dial discovered peer {}: {:?}", reg.record.peer_id(), err);
                        } else {
                            println!("Dialing discovered peer at: {:?}", addr);
                        }
                    }
                }
            }
            SwarmEvent::Behaviour(MyBehaviourEvent::Ping(event)) => {
                println!("Ping event: {event:?}");
            }
            SwarmEvent::Behaviour(MyBehaviourEvent::Identify(event)) => {
                println!("Identify event: {event:?}");
            }
            SwarmEvent::IncomingConnection { connection_id, local_addr, send_back_addr } => {
                println!("Incoming connection: {connection_id:?} from {send_back_addr} to {local_addr}");
            }
            SwarmEvent::OutgoingConnectionError { connection_id, peer_id, error } => {
                println!(
                    "Outgoing connection error: {:?}, peer_id: {:?}, error: {:#?}",
                    connection_id, peer_id, error
                );
            }
            SwarmEvent::ConnectionClosed {
                peer_id,
                connection_id,
                cause,
                endpoint,
                ..
            } => {
                println!(
                    "Connection closed with peer: {:?} on endpoint: {:?}, connection ID: {:?}",
                    peer_id, endpoint, connection_id
                );
                if let Some(error) = cause {
                    println!("Cause of closure: {:#?}", error);
                } else {
                    println!("Connection closed cleanly without an error.");
                }
            }
            SwarmEvent::IncomingConnectionError { connection_id, local_addr, send_back_addr, error } => {
                println!("Incoming connection error: {connection_id:?}, from {send_back_addr} to {local_addr}, error: {error:?}");
            }
            other => {
                println!("Unhandled event: {other:?}");
            }
        }
    }
}