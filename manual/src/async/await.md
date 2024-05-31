# Awaiting, Joining, Selecting

Async tasks run until they wait for another task or explicitly yield. There's a lot of different ways to wait for functions.

## Regular Await

You've seen this one:

```rust
async fn hello() {
    println!("Hi");
}

#[tokio::main]
async fn main() {
    hello().await
}
```

## Joining

Want to run a bunch of things at once and wait for them all to finish? `join!` is your friend:

```rust
use tokio::join;

async fn hello(n: i32) {
    println!("Hi {n}");
}

#[tokio::main]
async fn main() {
    join!(
        hello(1),
        hello(2),
        hello(3),
    );
}
```

## Joining a Vector of Futures

```rust
use tokio::task::JoinSet;

async fn double(n: i32) -> i32 {
    n * 2
}

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();

    for i in 0..10 {
        set.spawn(double(i));
    }

    while let Some(res) = set.join_next().await {
        println!("{}", res.unwrap());
    }
}
```

## Select - Receive From Multiple Sources

For example, you can listen for results from multiple channels at once:

```rust
async fn ticker(tx: tokio::sync::mpsc::Sender<i32>) {
    loop {
        tx.send(1).await;
        tokio::time::sleep(std::time::Duration::from_secs_f32(0.1)).await;
    }
}

async fn quitter(tx: tokio::sync::mpsc::Sender<i32>) {
        tokio::time::sleep(std::time::Duration::from_secs_f32(0.5)).await;
        tx.send(0).await;
}

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = tokio::sync::mpsc::channel::<i32>(10);
    let (tx2, mut rx2) = tokio::sync::mpsc::channel::<i32>(10);

    loop {
        tokio::select! {
            Some(val) = rx1.recv() => {
                println!("From 1: {val}");
            }
            Some(val) = rx2.recv() => {
                break;
            }
        }
    }
    println!("Quitting");
}
```

> This isn't limited to channels. Anything async that returns a result will work! It's common to listen to TCP traffic while tracking a global broadcast that its time to quit, for example.

You can do this with channels in sync Rust with the `crossbeam` crate and its `select` macro, too.