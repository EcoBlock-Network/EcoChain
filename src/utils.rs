use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use lazy_static::lazy_static;

lazy_static! {
    static ref COUNTER: Mutex<u64> = Mutex::new(0);
}

pub fn generate_unique_id() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();

    let mut counter = COUNTER.lock().unwrap();
    *counter += 1;
    format!("{}{}", now, counter)
}
