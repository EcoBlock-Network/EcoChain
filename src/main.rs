#[derive(Debug)]
struct Transaction {
    id: String,
    data: String,
}

fn main() {
    let tx = Transaction {
        id: "T1".to_string(),
        data: "Donnée de test".to_string(),
    };

    println!("Transaction créée {:?}", tx);
}
