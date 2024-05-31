# Chunk the Workload

We did this part earlier, so you should breeze through this task!

Your program should:

* Keep the timing, and `candidates` generation.
* Uncomment `MAX_THREADS` and use a scoped thread pool to generate that many chunks, and that many threads to iterate through a chunk of candidates.
* Store all of the results in a `Vec<usize>` just like the previous program. You'll need to protect it with a `Mutex` since multiple threads are `push`ing into it!
* Make sure you find 9592 primes!
* Don't sneakily use Rayon yet!

![](../images/ScrollTime.png)

Here's my version:

```rust
use std::time::Instant;
use std::thread;
use std::sync::Mutex;

const NUM_THREADS: usize = 10;
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
    let candidates: Vec<usize> = (0 .. MAX_NUMBER).collect();

    // Perform the calculation
    let start = Instant::now(); // We're not timing the initial creation
    let chunks = candidates.chunks(MAX_NUMBER / NUM_THREADS);
    let primes: Mutex<Vec<usize>> = Mutex::new(Vec::with_capacity(10000));
    thread::scope(|scope| {
        for chunk in chunks {
            scope.spawn(|| {
                let local_results: Vec<usize> = chunk
                    .iter()
                    .filter(|n| is_prime(**n))
                    .copied()
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

> On my desktop in the office, it ran in 0.1158 seconds. That's a decent improvement, but nowhere close to linear!

Some common issues:

* Are you locking and pushing each time? That's MUCH slower.
* Did you put `move ||` in your thread? You don't want to move the candidates into place! (You could get away with it by cloning an `Arc`---but you really don't need that!)