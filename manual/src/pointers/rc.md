# Rc

Rc stands for "reference counted". It's the `shared_ptr` you get in C++ if the standard library thinks that you won't be using threads. It's literally a tuple `(count, T)` in memory - and `count` is not thread safe (consequently, `Rc` is not `Send`---so you *can't* accidentally send it between threads).

It works just like a `shared_ptr`, including semantics for weak pointers if you need them. The same "don't make cycles" rules apply as C++.

```rust
use std::rc::Rc;

struct MyType;

impl Drop for MyType {
    fn drop(&mut self) {
        println!("I was dropped!");
    }
}

fn call_me(t: Rc<MyType>) {
    println!("Function called");
}

fn main() {
    let t = Rc::new(MyType);
    for _ in 0..10 {
        call_me(t.clone()); // Clone is how you make a new reference
    }
}