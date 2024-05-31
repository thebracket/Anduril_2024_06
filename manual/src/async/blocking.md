# Blocking

Async channels run like old multi-tasking computers - the old Mac OS (pre 10), Windows before 95, etc. Multitasking is *cooperative*---tasks run until they yield. If they don't yield, they can stall other tasks in the queue. Worse, if you somehow put the current *thread* into an idle state, the whole executor for that thread is also idle. The best advice I have is: don't do that.

## Locking Your Executor Thread

The most simple way to mess things up:

```rust
use std::time::Duration;

async fn counter(n: f32) {
    std::thread::sleep(Duration::from_secs_f32(1.0 - n));
    println!("{n}");
}

async fn correct_counter(n: f32) {
    tokio::time::sleep(Duration::from_secs_f32(1.0 - n)).await;
    println!("{n}");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    for i in 0..10 {
        tokio::spawn(correct_counter(i as f32 / 10.0));
    }

    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

If you run it calling `correct_counter` - the tasks sleep properly, are scheduled concurrently and it runs nicely. If you call the `counter` function, then the whole process stops inbetween every sleep call---which isn't what you want.

> If you'd called `recv` on a threaded channel, the whole program would freeze permanently!

So while you are in `async` land, use the async equivalent of functionality such as sleep, channels and similar. If you do want to send to a synchronous channel, make sure you are on a different thread - spawn the channel receiver into its own thread.

## Doing Too Much Work in a Task

```rust
use std::time::Duration;

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

async fn find_prime() {
    let result = is_prime(999983);
    println!("Is prime: {}", result);
}

async fn spin() {
    for _ in 0 .. 5 {
        println!("Doing some work");
        tokio::time::sleep(Duration::from_secs_f32(0.01)).await;
    }

}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let handle = tokio::spawn(spin());

    find_prime().await;

    let _ = handle.await;
}
```

This gives the result:

```
Is prime: true
Doing some work
Doing some work
Doing some work
Doing some work
Doing some work
```

The `is_prime` function is hogging the CPU---and we're in single threaded mode, so nothing else can execute.

### Yielding

It's not very performant, but you *can* decide to explicitly yield:

```rust
use std::time::Duration;

async fn is_prime(n: usize) -> bool {
    if n <= 1 {
        false
    } else {
        for div in 2 .. n {
            if n % div == 0 {
                return false;
            }
            tokio::task::yield_now().await;
        }
        true
    }
}

async fn find_prime() {
    let result = is_prime(999983).await;
    println!("Is prime: {}", result);
}

async fn spin() {
    for _ in 0 .. 5 {
        println!("Doing some work");
        tokio::time::sleep(Duration::from_secs_f32(0.01)).await;
    }

}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let handle = tokio::spawn(spin());

    find_prime().await;

    let _ = handle.await;
}
```

This is a good option if you are iterating through a large dataset and want to explicitly give other tasks a chance to run sometimes. The downside is that it is quite a bit slower.

### Spawn Blocking

If you have a sync task that takes a while and you want to call it from inside an async context, Tokio provides `spawn_blocking` for this:

```rust
use std::time::Duration;

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

async fn find_prime() {
    let result = tokio::task::spawn_blocking(|| is_prime(999983))
        .await.unwrap();
    println!("Is prime: {}", result);
}

async fn spin() {
    for _ in 0 .. 5 {
        println!("Doing some work");
        tokio::time::sleep(Duration::from_secs_f32(0.01)).await;
    }

}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let handle = tokio::spawn(spin());

    find_prime().await;

    let _ = handle.await;
}
```

`spawn_blocking` moves your call onto a... thread. It's running a thread, and wrapping it in an async facade. You don't lose the speed benefits, but you have to be a little careful not to completely overwhelm your CPU by spawning too many blocking calls.

> Tokio uses `spawn_blocking` a LOT internally. Many file-based IO functions that don't have an async implemention on a given platform will be spawned this way.