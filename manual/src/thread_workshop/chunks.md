# Chunking a CPU Bound Problem

A *really* common pattern is when you have a bunch of work to do, and you want to divide it between threads for faster calculation.

> Please follow along. We're bulding the basis for the next workshop.

Let's build a basic chunking solution. We'll start by collecting the numbers 0 to 999,999 into a vector of `u64` types:

```rust
fn main() {
    let workload: Vec<u64> = (0..1_000_000).collect();
}
```

> The ability to collect ranges into a vector is a great typing saver!

Now we want to divide it into chunks to work per-cpu. Vector has a `chunks(# of items in chunk)` function to help with this. It creates *references* to the original data:

```rust
fn main() {
    const NUM_CPUS: usize = 8;
    let workload: Vec<u64> = (0..1_000_000).collect();
    let chunks = workload.chunks(workload.len() / NUM_CPUS);

    // Check that the chunking worked
    for chunk in chunks {
        println!("{}", chunk.len());
    }
}
```

So this looks pretty reasonable, right?

```rust
use std::thread;
use std::thread::JoinHandle;

fn main() {
    const NUM_CPUS: usize = 8;
    let workload: Vec<u64> = (0..1_000_000).collect();
    let chunks = workload.chunks(workload.len() / NUM_CPUS);

    let mut handles: Vec<JoinHandle<u64>> = Vec::with_capacity(NUM_CPUS);
    for chunk in chunks {
        handles.push(thread::spawn(move || {
            chunk.iter().sum()
        }));
    }

    let total: u64 = handles.into_iter().map(|h| h.join()).flatten().sum();
    println!("{total}");
}
```

The problem is that it won't work. `workload` isn't *guaranteed* to outlive the workload. I found this midly frustrating when I started with Rust!