# What does Tokio::Main Do?

In async examples, you'll often see something like this:

```rust
async fn hello() {
    println!("Hello!");
}

#[tokio::main]
async fn main() {
    hello().await;
}
```

So what's going on here?

* `fn hello` has the `async` keyword.
    * This *changes* the function signature to return a `Future`.
* The `main` function is also `async`. 
    * This only works because of the procedural macro `#[tokio::main]` which: 
        * Quietly renameds `main`.
        * Adds a "real" `main` function that isn't async.
        * Calls `block_on` to execute the async `main` function you provided. We'll talk about `block_on` on the next slide.
* Notice that `hello()` is followed by `.await`.
    * If you just call `hello()`, it returns a `Future` and doesn't do anything!
    * `await`: 
        * Adds the `Future` to Tokio (the runtime)'s execution list.
        * Suspends the `main` function.
        * Calls the next available future, which will be `hello`.
        * When `hello` finishes, because it was called by `main`---main will be marked as "ready".
        * `main` then resumes.

## Types of Execution

Tokio is actually doing a LOT with its innocuous `tokio::main`. It spawns a runtime, which in turn spawns one work queue per CPU core. Each work queue implements work-stealing, so if another core's queue contains "ready" work, a free CPU can take it over and perform it.

You can also run Tokio completely single-threaded:

```rust
async fn hello() {
    println!("Hello!");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    hello().await;
}
```

This can be great if you want to isolate your runtime down to a small part of your program, or keep the footprint small.

> Futures are NOT threads. Futures are scheduled by the runtime, threads are scheduled by the operating system. You can have *many* pending Futures on a single thread's work list. Execution will switch whenever you `await` or explitily `yield_now`. It's cooperative multitasking, just like older computers.