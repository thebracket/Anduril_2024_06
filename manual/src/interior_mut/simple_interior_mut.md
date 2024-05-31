# Task: Simple Interior Mutable Structure

Let's build a simple structure with interior mutability. Follow the basic structure:

```rust
struct MyType {
    data: Mutex<MyData>,
}
```

Now implement a function that mutates `MyData` - it acquires a lock, changes the data, and releases the lock (on function or scope exit). Nothing fancy.

Next, add a shared instance of `MyType`---and have two scoped threads call the mutation function.

> Hint: You don't need `&mut self` in your function! In fact, it won't compile if you try.

![](../images/ScrollTime.png)

Here's my version:

```rust
use std::thread;
use std::sync::Mutex;

struct MyType {
    data: Mutex<i32>
}

impl MyType {
    fn add_one(&self) {
        let mut lock = self.data.lock().unwrap();
        *lock += 1;
    }
}

fn main() {
    let data = MyType { data: Mutex::new(1) };

    thread::scope(|scope| {
        for _ in 0 .. 2 {
            scope.spawn(|| {
                for _ in 0 .. 100 {
                    data.add_one();
                }
            });
        }
    });

    let lock = data.data.lock().unwrap();
    println!("{}", *lock);
}
```

## What Have We Acheived Here?

1. The interface to the structure is much nicer to work with!
2. You could now change `Mutex` to `RwLock` or any other locking primitive---and your users wouldn't be any the wiser.
3. Interior mutability really does work.
