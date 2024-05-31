# Task: Your First Channel

> **Hint**: Senders are designed to be cloned. It's lightweight and you can clone as many of them as you want.

Build a program that:

* Creates an MPSC channel.
* Iterates from 0 to 9 (0..10)
    * Creates a thread
        * Create a range of numbers in a vector from `(i*10)` to `(i*1000)`.
        * Sum the vector.
        * Sends the result into the channel.
* Receives a result from the channel 10 times, adding it to a grand total.
* Prints the total.

![](../images/ScrollTime.png)

Here's my version:

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let my_tx = tx.clone();
        std::thread::spawn(move || {
            let numbers: Vec<usize> = (i*10 .. i*1000).collect();
            let sum: usize = numbers.iter().sum();
            my_tx.send(sum).unwrap();
        });
    }

    let mut total = 0;
    for _ in 0..10 {
        total += rx.recv().unwrap();
    }
    println!("Total: {}", total);
}
```

In many ways, this is cleaner than having a set of thread scopes, parsing result handles and similar. It's also indicative of a very common workload: threads that do heavy calculations, and send the results to a "result handler". Channels keep you from having shared state everywhere.