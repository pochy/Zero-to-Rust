use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Store {
    data: HashMap<String, String>,
}

impl Store {
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }
}

fn main() {
    let mut store = Store::default();

    store.set("name".to_string(), "Rust".to_string());

    if let Some(value) = store.get("name") {
        println!("name = {}", value);
    }

    println!("exists lang = {}", store.get("lang").is_some());
    println!("deleted name = {}", store.delete("name"));
    println!("exists name = {}", store.get("name").is_some());
}
