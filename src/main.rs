use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use lazy_static::lazy_static;



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
    fn add_transaction(&mut self, data: String, approves: Vec<String>) {
        let id = generate_unique_id();
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

    // remove a transaction from the DAG
    fn remove_invalid_transaction(&mut self, id: &String) {
        if !self.validate_transaction(id) {
            println!("Transaction invalide détectée : {}", id);

            self.transactions.remove(id);

            let invalid_children: Vec<String> = self.transactions
                .iter()
                .filter(|(_, transaction)| transaction.approves.contains(id))
                .map(|(key, _)| key.clone())
                .collect();

            for child_id in invalid_children {
                self.remove_invalid_transaction(&child_id); 
            }
        }
    }
}

lazy_static! {
    static ref COUNTER: Mutex<u64> = Mutex::new(0);
}

fn generate_unique_id() -> String {
    let now = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_nanos();

    let mut counter = COUNTER.lock().unwrap();
    *counter += 1;
    format!("{}{}", now, counter)
}

fn main() {
    let mut dag = DAG::new();

    // Ajouter des transactions
    dag.add_transaction("Transaction 1".to_string(), vec![]);
    dag.add_transaction("Transaction 2".to_string(), vec![]);
    dag.add_transaction("Transaction 3".to_string(), vec!["12345".to_string()]);

    // Afficher les transactions
    println!("Transactions dans le DAG :");
    dag.display();
}