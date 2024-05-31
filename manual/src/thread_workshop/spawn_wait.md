# Task: Spawn Threads and Wait

Given the standard library thread syntax (don't use scoped threads if you already know about them!):

1. Create a container to hold thread handles.
2. Iterate from 0..10
    * Spawn a thread that prints "Hello from thread {x}" - where x is the thread number.
    * Add the thread's join handle to the handle container.
3. Iterate through the thread handles
    * Call `join()` on each one.

![](../images/ScrollTime.png)

Here's my version:

```rust
use std::thread;

fn main() {
    // It's good practice to make "magic numbers" a named constant.
    const N_THREADS: usize = 10;

    // If you know exactly how large a vector is, `with_capacity` lets you
    // specify exactly how much space the vector should use ahead of time.
    // Just like C++ vectors, `Vec` doubles in size every time it exceeds
    // its current capacity.
    let mut handles = Vec::with_capacity(N_THREADS);

    for x in 0 .. N_THREADS {
        // Using move || to move x into the closure.
        let handle = thread::spawn(move || println!("Hello from thread {x}"));
        handles.push(handle);
    }

    // Why `into_iter()` (`for handle in handles` is the same),
    // and not `for handle in &handles` or `for handle in handles.iter()`?
    //
    // `into_iter()` is destructive - it moves everything out of the container.
    // You can't use the container again - but since it's going to be destroyed,
    // that's fine.
    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
}
```