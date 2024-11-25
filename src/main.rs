use std::{collections::HashMap, hash::Hash, vec};

#[derive(Debug)]
struct Transaction {
    id: String,
    data: String,
    approves: Vec<String>,
}

#[derive(Debug)]
struct DAG {
    transactions : HashMap<String, Transaction>,
}

impl DAG {
    //create new empty DAG
    fn new() -> DAG {
        DAG {
            transactions: HashMap::new(),
        }
    }

    //add a transaction to the DAG
    fn add_transaction(&mut self, id: String, data: String, approves: Vec<String>) {
        let transaction = Transaction {
            id: id.clone(),
            data: data,
            approves,
        };
        self.transactions.insert(id, transaction);
    }

    //get a transaction from the DAG
    fn display(&self) {
        for (id, transaction) in &self.transactions {
            println!("Transaction id: {}, data: {}, approves {}", id, transaction.data, transaction.approves.join(", "));
        }
    }

    //validate the DAG
    fn validate_transaction(&self, id: &String) -> bool {
        if let Some(transaction) = self.transactions.get(id) {
            for approved_id in &transaction.approves {
                if !self.transactions.contains_key(approved_id) {
                    return false;
                }
            }
            true 
        } else {
            false 
        }
    }
}

fn main() {
    let mut dag = DAG::new();

    dag.add_transaction("T1".to_string(), "Transaction 1".to_string(), vec![]);
    dag.add_transaction(
        "T2".to_string(),
        "Transaction 2".to_string(),
        vec!["T1".to_string()],
    );
    dag.add_transaction(
        "T3".to_string(),
        "Transaction 3".to_string(),
        vec!["T1".to_string(), "T2".to_string()],
    );

    println!("T3 est valide ? {}", dag.validate_transaction(&"T3".to_string())); // true
    println!("T4 est valide ? {}", dag.validate_transaction(&"T4".to_string())); // false
}