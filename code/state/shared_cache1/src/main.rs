use std::sync::{Mutex, Arc};
use std::collections::HashMap;

struct Cache<K, V> {
    cache: Mutex<HashMap<K, V>>
}

impl<K,V> Cache<K,V> 
where K: Eq + std::hash::Hash, V: Clone
{
    pub fn new() -> Arc<Self> {
        Arc::new(Self { cache: Mutex::new(HashMap::new()) })
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let lock = self.cache.lock().unwrap();
        lock.get(key).cloned()
    }

    pub fn store(&self, key: K, value: V) {
        let mut lock = self.cache.lock().unwrap();
        lock.insert(key, value);
    }
}

fn main() {
    let shared_cache = Cache::new();

    shared_cache.store("Hello".to_string(), 12);
    if let Some(n) = shared_cache.get(&"Hello".to_string()) {
        println!("{n}");
    }
}