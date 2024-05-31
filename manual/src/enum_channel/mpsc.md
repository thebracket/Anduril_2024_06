# MPSC

MPSC stands for "Multi Producer, Single Consumer". It's a channel for sending data from any number of producers, to a single receiver. It's built in to the standard library, so no crates required.

Here's a simple example:

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    std::thread::spawn(move || {
        while let Ok(n) = rx.recv() {
            println!("{n}");
        }
        println!("Channel Closed");
    });
    
    for i in 0..10 {
        tx.send(i).unwrap(); // Error if the channel died
    }
    std::mem::drop(tx);
    std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
    println!("Done");
}
```

When a channel has no more transmitters, it closes. We used `drop` to simulate it going out of scope.

## Speed

Sending is *really* fast. `send` doesn't block, and channels try to be lock free when they can.

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    std::thread::spawn(move || {
        while let Ok(n) = rx.recv() {
            println!("{n}");
        }
        println!("Channel Closed");
    });
    
    for i in 0..10 {
        let start = std::time::Instant::now();
        tx.send(i).unwrap(); // Error if the channel died
        let elapsed = start.elapsed();
        println!("Send time: {} nanos", elapsed.as_nanos());
    }
    std::mem::drop(tx);
    std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
    println!("Done");
}
```

On the playground this gives me:

```
Send time: 5480 nanos
Send time: 940 nanos
Send time: 950 nanos
Send time: 990 nanos
Send time: 950 nanos
Send time: 920 nanos
Send time: 930 nanos
Send time: 950 nanos
Send time: 950 nanos
Send time: 970 nanos
```

Note the first one is always slower! The channel lazily initializes.

Receiving is bounded by the latency of a context switch, but if there are multiple items in the channel the receiver will keep receiving them. (You can use `try_recv` to not block on receive; otherwise the thread waits for a conditional to signal that there is work - it's idle).

## Bounded Channels

You can also use "bounded" channels that have a limited capacity. This is a great idea if you don't know exactly how many messages you might be sending. The `send` call will block until capacity is available, creating back-pressure and regulating control flow.

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::sync_channel(5);
    
    std::thread::spawn(move || {
        while let Ok(n) = rx.recv() {
            println!("{n}");
        }
        println!("Channel Closed");
    });
    
    for i in 0..10 {
        tx.send(i).unwrap(); // Error if the channel died
    }
    std::mem::drop(tx);
    std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
    println!("Done");
}
```