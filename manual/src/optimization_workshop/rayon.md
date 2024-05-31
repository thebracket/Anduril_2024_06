# So what about Rayon?

As a final note, we could have used Rayon for this.

```bash
cargo add rayon
```

And the code is *much* shorter:

```rust
use std::time::Instant;
use rayon::prelude::*;

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
    let start = Instant::now(); // We're not timing the initial creation
    
    let primes: Vec<&usize> = candidates
        .par_iter()
        .filter(|n| is_prime(**n))
        .collect();
    let elapsed = start.elapsed();

    // Results
    println!("Found {} primes", primes.len());
    println!("Calculated in {:.4} seconds", elapsed.as_secs_f32());
}
```

And---with no optimization effort---it's actually the fastest at 0.0549 seconds on my office workstation!

Why is Rayon so much faster?

1. Rayon is making the thread pool on startup, so we're not timing it!
2. Rayon maintains a work queue per thread, and "tasks" are being added to the work queues for existing threads.
3. If a thread hasn't got any work to do, it'll "steal" work from other threads. So the imbalanced threads doesn't matter---threads will acquire work to do if it is available.

So for this type of problem, *if* you can use Rayon in your architecture it's often easier and more performant. Not every architecture permits Rayon, and not every problem readily devolves into a Rayon-friendly technique. If it does, it's worth trying---you'll often have to work hard to beat its performance.
