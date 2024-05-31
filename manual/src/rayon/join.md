# Joining

`join` in Rayon terms splits a task into two (which can itself join if needs be), and waits for both of them.

> You'll see the word join a lot! You join a thread to wait for it. You join into two Rayon tasks to run them both at once. You join into async futures to run lots of them. It's a seriously overused word!

A simple example:

```rust
fn main() {
    rayon::join(|| println!("Hello"), || println!("World"));
}
```