# Interleaving

Interleaving the results looks really promising. You'd conceptually have candidates like this:

```
[0, 2, 4]
[1, 3, 5]
```

And so on --- so each thread has a nearly equal number of high and low value numbers. It's not even all that hard to implement:

```rust
fn interleave(data: &[usize], num_chunks: usize) -> Vec<Vec<&usize>> {
    let mut chunks = Vec::with_capacity(num_chunks);
    for _ in 0 .. num_chunks {
        chunks.push(Vec::new());
    }

    for (i, item) in data.iter().enumerate() {
        chunks[i % num_chunks].push(item);
    }

    chunks
}

// And later on
let chunks= interleave(&candidates, num_threads);
```

> The source code is in `code/optimization/interleaved/`.

```rust
use std::time::Instant;
use std::thread;
use std::sync::Mutex;

const MAX_NUMBER: usize = 100_000;

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        false
    } else {
        for div in 2 .. n {
            if n % div == 0 {
                return false;
            }
        }
        true
    }
}

fn interleave(data: &[usize], num_chunks: usize) -> Vec<Vec<&usize>> {
    let mut chunks = Vec::with_capacity(num_chunks);
    for _ in 0 .. num_chunks {
        chunks.push(Vec::new());
    }

    for (i, item) in data.iter().enumerate() {
        chunks[i % num_chunks].push(item);
    }

    chunks
}

fn main() {
    let num_threads = thread::available_parallelism().unwrap();
    let candidates: Vec<usize> = (0 .. MAX_NUMBER).collect();

    // Perform the calculation
    let start = Instant::now(); // We're not timing the initial creation
    //let chunks = candidates.chunks(MAX_NUMBER / num_threads);
    let chunks= interleave(&candidates, num_threads.into());
    let primes: Mutex<Vec<usize>> = Mutex::new(Vec::with_capacity(10000));
    thread::scope(|scope| {
        for chunk in chunks.iter() {
            scope.spawn(|| {
                let local_results: Vec<usize> = chunk
                    .iter()
                    .filter(|n| is_prime(***n))
                    .map(|n| **n)
                    .collect();

                let mut lock = primes.lock().unwrap();
                lock.extend(local_results);
            });
        }
    });
    let elapsed = start.elapsed();

    // Results
    let lock = primes.lock().unwrap();
    println!("Found {} primes", lock.len());
    println!("Calculated in {:.4} seconds", elapsed.as_secs_f32());
}
```

On my office workstation, I get `0.0752 seconds`. That's actually slighty *worse* than my initial results!

If we exclude the chunk calculation from the equation (move the chunk calculation above the start) it improves to `0.0750 seconds`. Still not all that great!