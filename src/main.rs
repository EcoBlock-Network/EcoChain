use std::error::Error;
use libp2p::{
    core::transport::upgrade,
    tcp::Config as TcpConfig,
    core::Transport,
};
use futures::prelude::*;
use libp2p::swarm::SwarmEvent;
use libp2p::{ping, Multiaddr};
use std::time::Duration;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id = libp2p::PeerId::from(local_key.public());

    println!("Local Peer ID: {}", local_peer_id);


    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_async_std()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::tls::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|_| ping::Behaviour::default())?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {addr}")
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => println!("{event:?}"),
            _ => {}
        }
    }
}



// fn main() {
//     let mut dag = DAG::new();

//     let id1 = dag.add_transaction("Transaction 1".to_string(), vec![]);
//     let id2 = dag.add_transaction("Transaction 2".to_string(), vec![id1.clone()]);
//     let id3 = dag.add_transaction("Transaction 3".to_string(), vec![id1.clone(), id2.clone()]);

//     for id in dag.transactions.keys().cloned().collect::<Vec<_>>() {
//         dag.calculate_cumulative_weight(&id);
//     }

//     println!("Transactions dans le DAG :");
//     dag.display();
// }
