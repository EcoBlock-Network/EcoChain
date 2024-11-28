use std::error::Error;
use std::time::Duration;
use futures::StreamExt;
use libp2p::{
    ping,
    identify,
    rendezvous::{self, server},
    tcp::Config as TcpConfig,
    Multiaddr,
    swarm::{NetworkBehaviour, SwarmEvent},
};
use tracing_subscriber::EnvFilter;

//* Define a custom NetworkBehaviour combining Identify, Rendezvous, and Ping protocols
#[derive(NetworkBehaviour)]
struct MyBehaviour {
    identify: identify::Behaviour,
    rendezvous: server::Behaviour,
    ping: ping::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //* Initialize logging
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    //* Generate local peer identity
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id = libp2p::PeerId::from(local_key.public());
    println!("Local Peer ID: {}", local_peer_id);

    //* Configure the Swarm with Identify, Rendezvous, and Ping behaviours
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_async_std() //* Async runtime
        .with_tcp(
            TcpConfig::default(),
            libp2p::tls::Config::new,
            || libp2p::yamux::Config::default(),
        )?
        .with_behaviour(|_| MyBehaviour {
            identify: identify::Behaviour::new(identify::Config::new(
                "ecoblock-example/1.0.0".to_string(),
                local_key.public(),
            )),
            rendezvous: server::Behaviour::new(server::Config::default()),
            ping: ping::Behaviour::new(ping::Config::new().with_interval(Duration::from_secs(1))),
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();

    //* Listen on a random TCP port
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    println!("Swarm listening on /ip4/0.0.0.0/tcp/0");

    //* Dial a remote peer if an address is provided
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed remote peer: {addr}");
    }

    //* Main event loop
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on: {address}");
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                println!("Connected to peer: {peer_id}");
            }
            SwarmEvent::Behaviour(MyBehaviourEvent::Rendezvous(
                server::Event::PeerRegistered { peer, registration },
            )) => {
                println!(
                    "Peer {} registered for namespace '{}'",
                    peer, registration.namespace
                );
            }
            SwarmEvent::Behaviour(MyBehaviourEvent::Rendezvous(
                server::Event::DiscoverServed {
                    enquirer,
                    registrations,
                },
            )) => {
                println!(
                    "Served peer {} with {} registrations",
                    enquirer,
                    registrations.len()
                );
            }
            SwarmEvent::Behaviour(MyBehaviourEvent::Ping(event)) => {
                println!("Ping event: {event:?}");
            }
            other => {
                println!("Unhandled event: {other:?}");
            }
        }
    }
}