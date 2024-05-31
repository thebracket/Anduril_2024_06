# Rust Data Race

Taking the same (broken) code into Rust:

```rust
fn main() {
    let mut counter = 0;
    std::thread::scope(|scope| {
        for i in 0..3 {
            scope.spawn(|| {
                for _i in 0..100_000 {
                    counter += 1;
                }
            });
        }
    });
    println!("{counter}");
}
```

This gets you the error message `cannot borrow *counter* as mutable more than once at a time`. Going back to what we said about `mut` being a mutual exclusion --- a compile time mutex --- this makes sense. We're asking Rust to mutably borrow `counter` more than once. That isn't allowed. Your race condition is now a compile time bug and won't corrupt data at runtime.

You *can* (at least until the next edition of Rust) make this fail:

```rust
fn main() {
    static mut COUNTER: usize = 0;
    std::thread::scope(|scope| {
        for i in 0..3 {
            scope.spawn(|| {
                for _i in 0..100_000 {
                    unsafe {
                        COUNTER += 1;
                    }
                }
            });
        }
    });
    unsafe {
        println!("{COUNTER}");
    }
}
```

And now you have the same bug. Please don't do that in production. Notice that we couldn't do it without adding `unsafe`.

`unsafe` doesn't actually mean your code isn't safe. It means that Rust couldn't verify that your code is safe, please look closely at it in review - because that's where Rust can no longer help you.

There are three common uses of `unsafe`:

* **JUST FINE**: Calling outside of Rust code (e.g. into a C library). This is fine, but isn't safe because Rust can't reach out and statically analyze the other language!
* **WARNING**: Implementing something that doesn't fit well with Rust's safety model, but can be verified. Rust's LinkedList does some of this internally - and offers a safe API.
* **BE CAREFUL**: An optimization. You can bypass bounds checking with `get_unchecked`. It's worth benchmarking to see if that actually helps (LLVM is amazingly good at eliding bounds checks in release builds). But if you *need* it, it's there. It's just the opposite default to C++, and makes you note it as potentially dangerous.
* **OH NO**. `Hey, watch this`. Good for demos, a really horrible idea for production code.