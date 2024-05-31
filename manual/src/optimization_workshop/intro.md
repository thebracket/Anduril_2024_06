# Optimization Workshop

We're going to build a deliberately CPU hungry and inefficient function, and then build a program that uses it on chunks of data. We covered the first part, so it should be easy---so then we'll start optimizing it and looking at the corner cases that can make it interesting!

Let's start with this single-threaded version (either copy or grab it from the GitHub repo):

```rust
use std::time::Instant;

//const NUM_THREADS: usize = 10;
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
    let primes: Vec<usize> = candidates
        .iter()
        .filter(|n| is_prime(**n))
        .map(|n| *n)
        .collect();
    let elapsed = start.elapsed();

    // Results
    println!("Found {} primes", primes.len());
    //println!("{primes:?}");
    println!("Calculated in {:.4} seconds", elapsed.as_secs_f32());
}
```

That's not very efficient at all. We're not worried about the `is_prime` function---we're ok with that being really inefficient, we want to have enough workload to justify using threads (and warm up some CPUs!).

Let's make it better.

> On my desktop in the office, it ran in 0.5702 seconds.