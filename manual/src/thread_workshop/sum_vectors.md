# Task: Sum vectors in threads

Now you have the building blocks to make the code we've worked on function/compile correctly.

Make a program that:

1. Creates a vector containing the numbers 0 .. 999_999
2. Divide the vector into equal chunks (specify the number of chunks)
3. Spawn off the number of chunks as threads, each working on a separate chunk and returning the `sum` of the numbers inside the chunk.
4. Sum the results.
5. Print the total.

![](../images/ScrollTime.png)

Here's my version:

```rust
use std::thread;
use std::thread::ScopedJoinHandle;

fn main() {
    const NUM_CPUS: usize = 8;
    let workload: Vec<u64> = (0..1_000_000).collect();
    let chunks = workload.chunks(workload.len() / NUM_CPUS);

    let total: u64 = thread::scope(|scope| {
        let mut handles: Vec<ScopedJoinHandle<u64>> = Vec::with_capacity(NUM_CPUS);
        for chunk in chunks {
            handles.push(scope.spawn(|| {
                chunk.iter().sum()
            }));
        }

        handles.into_iter().map(|h| h.join()).flatten().sum()
    });

    println!("{total}");
}
```

Here's my not being clever version with everything spelled out in steps:

> It's up to you whether you prefer the more "go"-like explicit steps or the more traditionally Rust-like condensed/functional approach. It's easier to debug the explicit version, the condensed version is often slightly faster and more "Rustacean".

```rust
use std::thread;
use std::thread::ScopedJoinHandle;

fn main() {
    const NUM_CPUS: usize = 8;
    let workload: Vec<u64> = (0..1_000_000).collect();
    let chunks = workload.chunks(workload.len() / NUM_CPUS);

    let total: u64 = thread::scope(|scope| {
        let mut handles: Vec<ScopedJoinHandle<u64>> = Vec::with_capacity(NUM_CPUS);
        for chunk in chunks {
            handles.push(scope.spawn(|| {
                let chunk_sum = chunk.iter().sum();
                println!("Chunk Sum: {chunk_sum}");
                chunk_sum
            }));
        }

        let mut sum = 0;
        for handle in handles {
            sum += handle.join().unwrap();
        }
        sum
    });

    println!("Grand Total: {total}");
}
```