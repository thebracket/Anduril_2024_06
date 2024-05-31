# Custom Pools

One part of Rayon that can be a bitter pill to swallow is having a global thread pool that uses all of your CPUs. Maybe you want to leave room for other threads? Maybe you want to divide some workload into its own little pool?

You can customize the global threadpool with one line:

```rust
fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(22).build_global().unwrap();
}
```

You can use this to install your own panic handlers, name threads, or just customize the size of the pool.

You can also make your own thread pools as needed:

```rust
fn main() {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(22).build().unwrap();
    pool.install(|| {
        // Rayon operations in this closure will use the pool you created.
    });
}
```