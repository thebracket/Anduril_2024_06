# Task: Measure Thread Execution Time

> It's pretty likely that you'll have figured out *why* we aren't getting the kind of linear performance improvement we hope for. Pretend you don't know!

Now take your program, and add a timer to each thread's execution and output the timer results for each chunk.

![](../images/ScrollTime.png)

My version looks like this:

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

fn main() {
    let num_threads = thread::available_parallelism().unwrap();
    let candidates: Vec<usize> = (0 .. MAX_NUMBER).collect();

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

My output is interesting:

```
Chunk calculated in 0.0038 seconds
Chunk calculated in 0.0109 seconds
Chunk calculated in 0.0165 seconds
Chunk calculated in 0.0222 seconds
Chunk calculated in 0.0227 seconds
Chunk calculated in 0.0285 seconds
Chunk calculated in 0.0347 seconds
Chunk calculated in 0.0353 seconds
Chunk calculated in 0.0353 seconds
Chunk calculated in 0.0359 seconds
Chunk calculated in 0.0407 seconds
Chunk calculated in 0.0415 seconds
Chunk calculated in 0.0464 seconds
Chunk calculated in 0.0528 seconds
Chunk calculated in 0.0542 seconds
Chunk calculated in 0.0574 seconds
Chunk calculated in 0.0610 seconds
Chunk calculated in 0.0658 seconds
Chunk calculated in 0.0700 seconds
Chunk calculated in 0.0721 seconds
Found 9592 primes
Calculated in 0.0728 seconds
```

Total time is worse because I'm printing in the threads. But what's interesting is that the chunks have a *huge* variance in execution time.

Now look at the `is_prime` function---it has MUCH more work to do for large numbers, and very little for small numbers.

---

Any suggestions?

![](../images/ScrollTime.png)

Common ideas include:

* Randomly shuffling the numbers.
* Interleaved chunks
    * This one is quite hard, chunk slices will always be contiguous pointers to the beginning and end of a chunk---in order. CPU cache *loves* contiguous data. So it's appealing, but often not the best.
* Mutex contention.
* Improving our vector allocations.
