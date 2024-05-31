# Task: Use All Your CPUs

Let's use all the CPUs we have available.

If you're using an old (pre 1.58 Rust), you have have run into the `num_cpus` crate to do this. It's now part of the standard library.

Now you can replace the `NUM_THREADS` constant with a variable in `main()`:

```rust
let num_threads = thread::available_parallelism().unwrap();
```

And change the instances you've used the constant. Run it now. It'll now use every CPU at your disposal.

![](../images/ScrollTime.png)

My version now reads like this:

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
    let num_threads = available_parallelism().unwrap();
    let candidates: Vec<usize> = (0 .. MAX_NUMBER).collect();

    // Perform the calculation
    let start = Instant::now(); // We're not timing the initial creation
    let chunks = candidates.chunks(MAX_NUMBER / num_threads);
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

> On my office machine - an i7 with 20 cores - that reduced runtime to 0.739 seconds. We're getting better! Unless you have fewer CPUs than 10, in which case it might be worse...