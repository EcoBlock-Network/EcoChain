use std::error::Error;
use libp2p::tcp::Config as TcpConfig; 
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    // generate local peer id
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id = libp2p::PeerId::from(local_key.public());

    println!("Local Peer ID: {}", local_peer_id);

    // config tcp transport
    let transport = TcpConfig::new();
    println!("TCP transport configured successfully!");

    Ok(())
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
