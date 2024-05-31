# Rayon: Easy Mode

We mentioned `Rayon`. Rayon is a lot like Intel Threaded Building Blocks - it provides a lot of functionality that is perfect, so long as it maps to what you need:

* It creates a thread pool, one thread per CPU by default.
* Rayon operates on `tasks`, like an async system - but without being async.
* Rayon offers a *lot* of easy helpers to quickly parallelize your code.
* The price of this is that you are giving up some control. Rayon can be heavily customized, but you are no longer in control of *everything*.

> Follow along, we're going to replcae the whole thread scope with "easy mode".

```rust
use rayon::prelude::*;

fn main() {
    // We'll start by making the workload again
    let workload: Vec<u64> = (0..1_000_000).collect();

    // Now we want to sum it
    let sum: u64 = workload
        .par_iter()
        .sum();
    println!("{sum}");
}
```