use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct LruCache {
    capacity: usize,
    values: HashMap<String, String>,
    recent: VecDeque<String>,
}

impl LruCache {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            values: HashMap::new(),
            recent: VecDeque::new(),
        }
    }

    fn get(&mut self, key: &str) -> Option<&str> {
        if self.values.contains_key(key) {
            self.mark_recent(key);
        }
        self.values.get(key).map(String::as_str)
    }

    fn put(&mut self, key: String, value: String) {
        self.values.insert(key.clone(), value);
        self.mark_recent(&key);
        self.evict_if_needed();
    }

    fn mark_recent(&mut self, key: &str) {
        self.recent.retain(|existing| existing != key);
        self.recent.push_back(key.to_string());
    }

    fn evict_if_needed(&mut self) {
        while self.values.len() > self.capacity {
            if let Some(oldest) = self.recent.pop_front() {
                self.values.remove(&oldest);
            }
        }
    }
}

fn main() {
    let mut cache = LruCache::new(3);

    cache.put("a".to_string(), "1".to_string());
    cache.put("b".to_string(), "2".to_string());
    cache.put("c".to_string(), "3".to_string());
    println!("get a: {:?}", cache.get("a"));
    cache.put("d".to_string(), "4".to_string());

    println!("a: {:?}", cache.get("a"));
    println!("b: {:?}", cache.get("b"));
    println!("c: {:?}", cache.get("c"));
    println!("d: {:?}", cache.get("d"));
}
