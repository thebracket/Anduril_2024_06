# Sending Function Pointers

Being able to send just about anything over channels is incredibly powerful. You can send function pointers, and be part-way to writing your own Rayon (obviously, you'd use a more generic function type than this example):

```rust
use std::sync::mpsc;

enum Command {
    Execute {
        n: i32,
        func: fn (i32),
    },
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel();
    
    std::thread::spawn(move || {
        while let Ok(cmd) = rx.recv() {
            match cmd {
                Command::Execute{n, func} => func(n*2),
                Command::Quit => break,
            }
        }
        println!("Channel Closed");
    });
    
    for i in 0..10 {
        tx.send(
            Command::Execute {
                n: i,
                func: |i| println!("{i}"),
            }
        ).unwrap();
    }
    tx.send(Command::Quit).unwrap();
    std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
    println!("Done");
}
```