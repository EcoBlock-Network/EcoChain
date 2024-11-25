mod dag;
mod utils;

use dag::DAG;

fn main() {
    let mut dag = DAG::new();

    let id1 = dag.add_transaction("Transaction 1".to_string(), vec![]);
    let id2 = dag.add_transaction("Transaction 2".to_string(), vec![id1.clone()]);
    let id3 = dag.add_transaction("Transaction 3".to_string(), vec![id1.clone(), id2.clone()]);

    for id in dag.transactions.keys().cloned().collect::<Vec<_>>() {
        dag.calculate_cumulative_weight(&id);
    }

    println!("Transactions dans le DAG :");
    dag.display();
}
