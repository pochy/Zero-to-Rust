use std::collections::BTreeMap;

#[derive(Debug)]
struct Event {
    timestamp: u64,
    message: String,
}

fn main() {
    let mut index = BTreeMap::new();

    for timestamp in [100, 120, 130, 160, 200, 240] {
        index.insert(
            timestamp,
            Event {
                timestamp,
                message: format!("event at {timestamp}"),
            },
        );
    }

    println!("range 120..=200:");
    for (_timestamp, event) in index.range(120..=200) {
        println!("{} {}", event.timestamp, event.message);
    }
}
