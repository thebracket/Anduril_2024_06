# Task: Fan-Out Interior Mutability

Next, extend your `MyData` structure to include a second `Mutex`-protected entry, and add a second method to update the *other* data element.

Adjust your second thread to call the *second* update method.

![](../images/ScrollTime.png)

Here's my version:

```rust
use std::thread;
use std::sync::Mutex;

struct MyType {
    data: Mutex<i32>,
    data2: Mutex<i32>,
}

impl MyType {
    fn add_one(&self) {
        let mut lock = self.data.lock().unwrap();
        *lock += 1;
    }

    fn add_other(&self) {
        let mut lock = self.data2.lock().unwrap();
        *lock += 1;
    }
}

fn main() {
    let data = MyType { 
        data: Mutex::new(1),
        data2: Mutex::new(0),
    };

    thread::scope(|scope| {
        scope.spawn(|| {
            for _ in 0 .. 100 {
                data.add_one();
            }
        });
        scope.spawn(|| {
            for _ in 0 .. 100 {
                data.add_other();
            }
        });
    });

    let lock = data.data.lock().unwrap();
    let lock2 = data.data2.lock().unwrap();
    println!("{}", *lock);
    println!("{}", *lock2);
}
```

## What Have We Learned Here?

* You can individually protect parts of your structure.
* This is perfect when you want to "fan out" updates to a structure. Different workers can individually lock just the part they are working on---without stalling the whole pipeline.

## How Can We Improve This?

* In a real pipeline, you'd probably wrap `MyStruct` in an `Arc` - so there's only a single instance, and ownership can be shared.
    * Otherwise, you may have to wait for parts of the task to complete.
    * You've avoided having to `clone` copies of the whole data-set to work on just part of it.
    * You can still use `Drop` to finalize, firing only when all systems are done with it.
