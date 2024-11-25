use std::collections::HashMap;

use crate::utils::generate_unique_id;

#[derive(Debug)]
pub struct Transaction {
    pub id: String,
    pub data: String,
    pub approves: Vec<String>,
    pub cumulative_weight: u64,
}

#[derive(Debug)]
pub struct DAG {
    pub transactions: HashMap<String, Transaction>,
}

impl DAG {
    pub fn new() -> DAG {
        DAG {
            transactions: HashMap::new(),
        }
    }

    pub fn add_transaction(&mut self, data: String, approves: Vec<String>) -> String {
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

    pub fn display(&self) {
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

    pub fn validate_transaction(&self, id: &String) -> bool {
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

    pub fn remove_invalid_transaction(&mut self, id: &String) {
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

    pub fn calculate_cumulative_weight(&mut self, id: &String) -> u64 {
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
