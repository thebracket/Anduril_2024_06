# Threads Workshop

In this workshop, we're going to write a multi-threaded program. It's brave to workshop threads at the beginning of a workshop: threads are hard. Rust makes them a *lot* easier and safer.

You can create a thread with `std::thread::spawn`. It returns a "join handle" - just like C++.

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| println!("Hello From Thread"));
    handle.join().unwrap();
}
```

> If you don't `join()` the thread, the main function won't wait for it to finish before terminating. So there's no guarantee that the thread will execute at all.

---

Your closures can *capture*, but you are subject to both borrow-checking and lifetime rules.

This code won't compile because of lifetimes. `i` isn't guaranteed to be freed before `main` terminates:

```rust
use std::thread;

fn main() {
    let i = 5;
    let handle = thread::spawn(|| println!("Hello From Thread {i}"));
    handle.join().unwrap();
}
```

These work:

```rust
use std::thread;

fn main() {
    let i = 5;
    let handle = thread::spawn(move || println!("Hello From Thread {i}"));
    handle.join().unwrap();

    static J: i32 = 6;
    let handle = thread::spawn(|| println!("Hello From Thread {J}"));
    handle.join().unwrap();
}
```

Comments:
* The first one adds `move` to the closure. Moving ownership into the closure. Remember that for `Copy` types, this is the same as copying.
* The second one uses a `static`---not ideal, but it works.