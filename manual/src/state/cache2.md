# Task: Add Last Accessed and Size

Let's make it a least-recently used cache!

* Adjust the generics to require that the `K` also be cloneable.
* Instead of `HashMap<K,V>` store `HashMap<K,(Instant, V)>`
* Add a `capacity` field, and adjust the constructor to require it.
* The `store` function should check if there is capacity remaining and remove the oldest if there is not.

(This one gets a little tough, so let me know if we need to work through it)

![](../images/ScrollTime.png)

Here's my version:

```rust
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::time::Instant;

struct Cache<K, V> {
    cache: Mutex<HashMap<K, (Instant, V)>>,
    capacity: usize,
}

impl<K, V> Cache<K, V> 
where K: Eq + std::hash::Hash + Clone + std::fmt::Debug, V: Clone
{
    pub fn new(capacity: usize) -> Arc<Self> {
        Arc::new(Self { capacity, cache: Mutex::new(HashMap::new()) })
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let lock = self.cache.lock().unwrap();
        if let Some((_, value)) = lock.get(key) {
            Some(value.clone())
        } else {
            None
        }
    }

    pub fn store(&self, key: K, value: V) {
        let mut lock = self.cache.lock().unwrap();

        if lock.len() > self.capacity {
            let mut oldest_key = None;
            let mut oldest_time = Instant::now();

            for (k, (time, _)) in lock.iter() {
                if *time < oldest_time {
                    oldest_time = *time;
                    oldest_key = Some(k.clone());
                }
            }

            if let Some(k) = oldest_key {
                // Added print for demo
                println!("Evicting key: {:?}", k);
                lock.remove(&k);
            }
        }

        lock.insert(key, (Instant::now(), value));
    }
}

fn main() {
    let shared_cache = Cache::new(1);

    for i in 0..5 {
        let key = format!("Key {i}");
        shared_cache.store(i, key);
    }
}
```