mod dag;
mod utils;

use dag::DAG;
use libp2p::{identity, PeerId, Swarm, Multiaddr};

fn main(){
    //generate keypair
    let local_keys = identity::Keypair::generate_ed25519();

    //create peer id
    let local_peer_id = libp2p::PeerId::from(local_keys.public());

    println!("Local peer id: {:?}", local_peer_id);

    //config swarm

    let swarm = liubp2p::Swarm::new(
        libp2p::development_transport(local_keys).unwrap(),
        (),
        local_peer_id.clone(),
    );
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
