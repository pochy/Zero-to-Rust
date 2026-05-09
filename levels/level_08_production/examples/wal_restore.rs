use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Default)]
struct Store {
    data: HashMap<String, String>,
}

impl Store {
    fn apply(&mut self, line: &str) {
        let parts: Vec<&str> = line.trim().splitn(3, ' ').collect();

        match parts.as_slice() {
            ["SET", key, value] => {
                self.data.insert((*key).to_string(), (*value).to_string());
            }
            ["DEL", key] => {
                self.data.remove(*key);
            }
            _ => {}
        }
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}

fn restore_from_wal(content: &str) -> Store {
    let mut store = Store::default();

    for line in content.lines() {
        store.apply(line);
    }

    store
}

fn main() -> Result<(), io::Error> {
    let wal_path = env::temp_dir().join("zero_to_rust_demo.wal");
    let wal_content = "\
SET old value
DEL old
SET name Rust
SET lang std
";

    fs::write(&wal_path, wal_content)?;
    let content = fs::read_to_string(&wal_path)?;
    let store = restore_from_wal(&content);

    if let Some(value) = store.get("name") {
        println!("restored name = {}", value);
    }

    if let Some(value) = store.get("lang") {
        println!("restored lang = {}", value);
    }

    println!("exists old = {}", store.exists("old"));

    let _ = fs::remove_file(wal_path);

    Ok(())
}
