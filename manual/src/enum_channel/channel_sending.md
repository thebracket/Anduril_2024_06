# Sending Enums with Channels

You can send *any* type over a channel so long as it supports being moved between threads (implements the `Send` type). Most types are `Send` safe. Since this introduces a `memcpy`, for large structures it's sometimes faster to send a pointer (`Box`, `Arc`, etc.) than to send the actual data.

A *really* powerful pattern is sending enumerations, creating a "command channel" pattern:

```rust
use std::sync::mpsc;

enum Command {
    Print(i32),
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel();
    
    std::thread::spawn(move || {
        while let Ok(cmd) = rx.recv() {
            match cmd {
                Command::Print(n) => println!("{n}"),
                Command::Quit => break,
            }
        }
        println!("Channel Closed");
    });
    
    for i in 0..10 {
        tx.send(Command::Print(i)).unwrap(); // Error if the channel died
    }
    tx.send(Command::Quit).unwrap();
    std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
    println!("Done");
}
```