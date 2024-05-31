# Unit of Work

So the "unit of work" here is a thread itself. An actual, Operating System created thread --- with all the overhead that carries (it's best for CPU bound operations).

When you use `std::thread`, there are no termination guarantees (ignoring the halting problem!)---Rust won't be able to trace a thread down to a given `join` call and build lifetime assumptions based on when that thread will terminate.

> Regular `std::thread::spawn` is best for long-running threads that tick away doing something in the background.

As we saw in the intro, this isn't going to work:

```rust
use std::thread;

fn main() {
    let i = 5;
    let handle = thread::spawn(|| println!("Hello From Thread {i}"));
    handle.join().unwrap();
}
```

That's because Rust builds a *lifetime* for the reference to `i`---and can't guarantee that your thread isn't trying to access `i` after it leaves scope. You can make `i` "static"---but then you have initialization order and "sharing" issues to worry about. Sometimes, that's what you want. But if you're just trying to divide a workload up between threads, it's not great.