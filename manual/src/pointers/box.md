# Box

`Box` is directly equivalent to `unique_ptr`. A single pointer to an object on the heap, with a single owner.

Hopefully you're familiar with this. You have to *move* boxes around, just like you move `unique_ptr`. Ownership is in no way shared.

```rust
struct MyType;

impl Drop for MyType {
    fn drop(&mut self) {
        println!("I was dropped!");
    }
}

fn move_me(t: Box<MyType>) {
    println!("Function called");
}

fn main() {
    let t = Box::new(MyType);
    move_me(t);
}
```