use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
struct Transaction {
    id: String,
    data: String,
}

#[derive(Debug)]
struct DAG {
    transations : HashMap<String, Transaction>,
}

impl DAG {
    //create new empty DAG
    fn new() -> DAG {
        DAG {
            transations: HashMap::new(),
        }
    }

    //add a transaction to the DAG
    fn add_transaction(&mut self, id: String, data: String) {
        let transaction = Transaction {
            id: id.clone(),
            data: data,
        };
        self.transations.insert(id, transaction);
    }

    //get a transaction from the DAG
    fn display(&self) {
        for (id, transaction) in &self.transations {
            println!("Transaction id: {}, data: {}", id, transaction.data);
        }
    }
}

fn main() {
    let mut dag = DAG::new();
    dag.add_transaction("1".to_string(), "Transaction 1".to_string());
    dag.add_transaction("2".to_string(), "Transaction 2".to_string());
    dag.add_transaction("3".to_string(), "Transaction 3".to_string());
    dag.display();
}
