# What does Block On mean?

So `tokio::main` really makes a function that calls "block_on". What does that mean?

Remember that `sync` code can't call `async` code---and `async` code can't run without a runtime. `block_on` is how you *enter* the runtime. You hand it a function to start with, and the async code runs from there. When the async code is done, `block_on` returns control.

The code that's generated looks like this:

```rust
use tokio::runtime::Runtime;

async fn hello() {
    println!("Hello");
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(hello()); // Note that you execute the function to get the Future
}
```

That's the entry point to pretty much every async system (other than Embassy, which does it on boot!). 

## Configuring the Runtime

You may not want Tokio to use your entire system, but you may want more than one thread. You can customize just about *everything* about Tokio if you need to:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::runtime;

async fn hello() {
    println!("Hello from async");
}

fn thread_namer() -> String {
    static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
    let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
    format!("my-pool-{id}")
}

fn main() {
    let rt = runtime::Builder::new_multi_thread()
        // YOU DON'T HAVE TO SPECIFY ANY OF THESE
        .worker_threads(4)  // 4 threads in the pool
        .thread_name_fn(thread_namer) // Name the threads. 
                                     // This helper names them "my-pool-#" for debugging assistance.
        .thread_stack_size(3 * 1024 * 1024) // You can set the stack size
        .event_interval(61) // You can increase the I/O polling frequency
        .global_queue_interval(61) // You can change how often the global work thread is checked
        .max_blocking_threads(512) // You can limit the number of "blocking" tasks
        .max_io_events_per_tick(1024) // You can limit the number of I/O events per tick
        // YOU CAN REPLACE THIS WITH INDIVIDUAL ENABLES PER FEATURE
        .enable_all()
        // Build the runtime
        .build()
        .unwrap();

    rt.block_on(hello());
}
```

## Spawning into Existing Runtimes

Once you have a runtime, you can pass it around and use it as an interface for synchronous code:

```rust
async fn hello(i: i32) {
    println!("Starting {i}");
    tokio::time::sleep(std::time::Duration::from_secs_f32(0.01)).await;
    println!("{i}");
}

fn main() {
    use tokio::runtime::Runtime;

    let rt = Runtime::new().unwrap();

    let my_rt = rt.handle(); // Handle lets you get another link to the existing runtime
    for i in 0 .. 10 {
        // We're using "spawn" to *add* a task to the runtime.
        // This does not block our current thread!
        my_rt.spawn(hello(i));

        // Announce that the current thread continues
        println!("Sync {i}");
    }

    // Perform the actual block-on to hand over control to Tokio for a second.
    rt.block_on(async {
        println!("Starting Block On");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("Done");
    });
}
```

Notice that nothing runs until you `block_on` --- you're storing up futures but they aren't started, because the runtime isn't running yet.

This is also unwieldy: you can run into lifetime "fun" with grabbing handles in this way, and spawning directly into them.

## Use a Channel

A bettter approach is to use an `async` channel; Tokio's async channels provide a `sync` interface just for this:

```rust
#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(10); // Bounded

    std::thread::spawn(move || {
        for i in 0 .. 10 {
            tx.blocking_send(i);
        }
    });

    while let Some(message) = rx.recv().await {
        println!("Received {message} from thread land");
    }
    println!("Channel dropped - no senders remain.");
}
```

Just like sync channels, you can send a channel for replies---giving you two-way communication between sync and async land.

> Never feel like you have to make your whole program async. For number crunching and complex CPU-bound workloads, sync Rust is much more pleasant to work with! But async can really shine for networking.
