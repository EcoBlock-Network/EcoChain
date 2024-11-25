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
    }}

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
        dag.add_transaction(
            "T4".to_string(),
            "Transaction 4".to_string(),
            vec!["T3".to_string(), "T5".to_string()], // T5 n'existe pas
        );
    
        println!("Transactions avant suppression :");
        dag.display();
    
        dag.remove_invalid_transaction(&"T4".to_string());
    
        println!("\nTransactions après suppression :");
        dag.display();
    }