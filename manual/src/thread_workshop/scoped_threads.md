# Scoped Threads

*Scoped threads* were added to the standard library to solve this problem. Scoped threads spawn inside a scope---and the scope will not terminate until all the threads are done. That makes it useless for spawning long-lived threads, and ideal for "fan out" calculations that you want to divide between CPUs.

So while this gives you lifetime problems:

```rust
use std::thread;

fn main() {
    let i = 5;
    thread::spawn(|| println!("{i}"));
}
```

This doesn't:

```rust
use std::thread;

fn main() {
    let i = 5;
    thread::scope(|scope| {
        scope.spawn(|| println!("{i}"));
    });
}
```

> Note that `i` is *outside* the scope.

You don't *have* to `join()` at the end of a scope. If you don't, the scope itself will. It *guarantees* that all threads made with the `scope` parameter have terminated before the scope continues. This allows the lifetime calculator to be *absolutely sure* that captured variables from the parent scope will outlive the thread scope---so you can reference them.

You *can* still join:

```rust
use std::thread;
use std::thread::ScopedJoinHandle;

fn main() {
    let i = 5;
    thread::scope(|scope| {
        // Note: you don't need to specify the type. But sometimes you do,
        // so it's here to help you.
        let handle: ScopedJoinHandle<()> = scope.spawn(|| format!("{i}"));

        println!("{}", handle.join().unwrap());
    });
}
```