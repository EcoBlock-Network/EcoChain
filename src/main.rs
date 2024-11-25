use std::{collections::HashMap, hash::Hash, vec};

#[derive(Debug)]
struct Transaction {
    id: String,
    data: String,
    approvers: Vec<String>,
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
    fn add_transaction(&mut self, id: String, data: String, approvers: Vec<String>) {
        let transaction = Transaction {
            id: id.clone(),
            data: data,
            approvers,
        };
        self.transations.insert(id, transaction);
    }

    //get a transaction from the DAG
    fn display(&self) {
        for (id, transaction) in &self.transations {
            println!("Transaction id: {}, data: {}, Approvers {}", id, transaction.data, transaction.approvers.join(", "));
        }
    }

    //validate the DAG
    fn validate_transaction(&self, id: String) -> bool {
        let transaction = self.transations.get(&id);
        match transaction {
            Some(transaction) => {
                for approver in &transaction.approvers {
                    if !self.transations.contains_key(approver) {
                        return false;
                    }
                }
                return true;
            }
            None => return false,
        }
    }
}

fn main() {
    // Créer un DAG
    let mut dag = DAG::new();

    // Ajouter des transactions avec des liens
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

    // Valider des transactions
    println!("T3 est valide ? {}", dag.validate_transaction(&"T3".to_string())); // true
    println!("T4 est valide ? {}", dag.validate_transaction(&"T4".to_string())); // false
}