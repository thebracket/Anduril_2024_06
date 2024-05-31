# Replying

You can even send channel senders over channels. So if your call requires a response, you could provide a mechanism over which to reply and have the other thread do the work while you wait (or poll a set of channels) for a response:

```rust
use std::sync::mpsc;

enum Command {
    Execute {
        n: i32,
        func: fn (i32) -> i32,
        reply: mpsc::Sender<i32>,
    },
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel();
    
    std::thread::spawn(move || {
        while let Ok(cmd) = rx.recv() {
            match cmd {
                Command::Execute{n, func, reply} => {
                    let response = func(n*2);
                    reply.send(response).unwrap();
                }
                Command::Quit => break,
            }
        }
        println!("Channel Closed");
    });
    
    for i in 0..10 {
        let (reply_tx, reply_rx) = mpsc::channel();
        tx.send(
            Command::Execute {
                n: i,
                func: |i| i*3,
                reply: reply_tx,
            }
        ).unwrap();
            
        if let Ok(response) = reply_rx.recv() {
            println!("Response received: {response}");
        }
    }
    tx.send(Command::Quit).unwrap();
    std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
    println!("Done");
}
```

You're part way to implementing the actor model here, or you could be having results sent off to a completely different handler. You can even `Mutex` lock a receiver and have multiple threads await a lock and try to receive - for a poor mans work stealing. (For *good* work stealing, use the `crossbeam` crate)