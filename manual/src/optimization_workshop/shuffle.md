# Task: Shuffle the Deck

We're going to go ahead and randomly shuffle our list of candidates. You can do this with the `rand` crate and the `rand::seq::SliceRandom` trait that enables a `shuffle` method:

Since you need 2 new lines of code, and 1 change - I'll just show you my answer:

```rust
use std::time::Instant;
use std::thread;
use std::sync::Mutex;

// ADD THIS:
use rand::seq::SliceRandom;

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

fn main() {
    let num_threads = thread::available_parallelism().unwrap();
    // ADD MUT
    let mut candidates: Vec<usize> = (0 .. MAX_NUMBER).collect();
    // CALL SHUFFLE
    candidates.shuffle(&mut rand::thread_rng());

    // Perform the calculation
    let start = Instant::now(); // We're not timing the initial creation
    let chunks = candidates.chunks(MAX_NUMBER / num_threads);
    let primes: Mutex<Vec<usize>> = Mutex::new(Vec::with_capacity(10000));
    thread::scope(|scope| {
        for chunk in chunks {
            scope.spawn(|| {
                let start = Instant::now();
                let local_results: Vec<usize> = chunk
                    .iter()
                    .filter(|n| is_prime(**n))
                    .copied()
                    .collect();

                let mut lock = primes.lock().unwrap();
                lock.extend(local_results);
                let elapsed = start.elapsed();
                println!("Chunk calculated in {:.4} seconds", elapsed.as_secs_f32());
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

The results are a LOT better:

```
Chunk calculated in 0.0391 seconds
Chunk calculated in 0.0399 seconds
Chunk calculated in 0.0422 seconds
Chunk calculated in 0.0423 seconds
Chunk calculated in 0.0462 seconds
Chunk calculated in 0.0489 seconds
Chunk calculated in 0.0500 seconds
Chunk calculated in 0.0514 seconds
Chunk calculated in 0.0536 seconds
Chunk calculated in 0.0549 seconds
Chunk calculated in 0.0549 seconds
Chunk calculated in 0.0561 seconds
Chunk calculated in 0.0564 seconds
Chunk calculated in 0.0571 seconds
Chunk calculated in 0.0573 seconds
Chunk calculated in 0.0569 seconds
Chunk calculated in 0.0579 seconds
Chunk calculated in 0.0576 seconds
Chunk calculated in 0.0590 seconds
Chunk calculated in 0.0605 seconds
Found 9592 primes
Calculated in 0.0610 seconds
```

If we comment out the per-thread timing and printing (measuring adds overhead!), it's slightly better yet:

```
Found 9592 primes
Calculated in 0.0608 seconds
```

You'll notice that I didn't include the shuffle in my timing. If I add it back in, my office workstation yielded `0.0609 seconds`. Shuffling is *fast*! It's also often a great strategy when you don't have control over the order of inputs and have an algorithm greatly affected by the relative value of the input.