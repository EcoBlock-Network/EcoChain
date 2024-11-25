use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use lazy_static::lazy_static;

#[derive(Debug)]
struct Transaction {
    id: String,
    data: String,
    approves: Vec<String>,
    cumulative_weight: u64, 
}

#[derive(Debug)]
struct DAG {
    transactions: HashMap<String, Transaction>,
}

impl DAG {
    fn new() -> DAG {
        DAG {
            transactions: HashMap::new(),
        }
    }

    fn add_transaction(&mut self, data: String, approves: Vec<String>) -> String {
        let id = generate_unique_id();
        let transaction = Transaction {
            id: id.clone(),
            data,
            approves,
            cumulative_weight: 1,
        };
        self.transactions.insert(id.clone(), transaction);
        id
    }

    fn display(&self) {
        for (id, transaction) in &self.transactions {
            println!(
                "Transaction id: {}, data: {}, approves: {}, cumulative_weight: {}",
                id,
                transaction.data,
                transaction.approves.join(", "),
                transaction.cumulative_weight
            );
        }
    }

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

    fn calculate_cumulative_weight(&mut self, id: &String) -> u64 {
        let approved_ids = if let Some(transaction) = self.transactions.get(id) {
            transaction.approves.clone()
        } else {
            return 0; 
        };
    
        let mut weight = 1;
    
        for approved_id in approved_ids {
            weight += self.calculate_cumulative_weight(&approved_id);
        }
    
        if let Some(transaction) = self.transactions.get_mut(id) {
            transaction.cumulative_weight = weight;
        }
    
        weight
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

    let id1 = dag.add_transaction("Transaction 1".to_string(), vec![]);
    let id2 = dag.add_transaction("Transaction 2".to_string(), vec![id1.clone()]);
    let id3 = dag.add_transaction("Transaction 3".to_string(), vec![id1.clone(), id2.clone()]);

    for id in dag.transactions.keys().cloned().collect::<Vec<_>>() {
        dag.calculate_cumulative_weight(&id);
    }

    println!("Transactions dans le DAG :");
    dag.display();
}
