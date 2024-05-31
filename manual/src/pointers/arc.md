# Arc

Arc stands for "atomic reference counted". It's the `shared_ptr` you get in C++ if the standard library thinks that you will be using threads.

It works just like a `shared_ptr`, including semantics for weak pointers if you need them. The same "don't make cycles" rules apply as C++.

`Arc` is exactly the same as `Rc`, but uses an atomic for the reference count. So you can send it between threads.

```rust
use std::sync::Arc;

struct MyType;

impl Drop for MyType {
    fn drop(&mut self) {
        println!("I was dropped!");
    }
}

fn call_me(t: Arc<MyType>) {
    println!("Function called");
}

fn main() {
    let t = Arc::new(MyType);
    for _ in 0..10 {
        call_me(t.clone()); // Clone is how you make a new reference
    }
}