# Channels - Concept

How many of you have ever written some code that uses a shared variable to pass state between threads?

```rust
use std::sync::Mutex;

fn main() {
    static WORKLOAD: Mutex<Vec<usize>> = Mutex::new(Vec::new());
    let mut handles = Vec::new();
    
    for _ in 0..2 {
        let handle = std::thread::spawn(|| {
            loop {
                let mut lock = WORKLOAD.lock().unwrap();
                if lock.is_empty() {
                    std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
                    if lock.is_empty() {
                        println!("Nothing to do - stopping");
                        break;
                    }
                } else {
                    while let Some(n) = lock.pop() {
                        println!("{n}");
                    }
                }
            }
        });
        handles.push(handle);
    }
    
    for i in 0 ..= 10 {
        let mut lock = WORKLOAD.lock().unwrap();
        lock.push(i);
    }
    
    for h in handles {
        h.join().unwrap();
    }
}
```

This isn't too horrible, but it quickly becomes spaghetti on a large code base. *Channels* are a mechanism for sending data between threads (or tasks, or futures...). It can lead to a much cleaner pattern for this type of management.