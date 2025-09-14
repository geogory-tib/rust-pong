use std::time::{self, UNIX_EPOCH};

pub fn rand(delim: u64) -> u64 {
    let time = time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    time % delim
}
