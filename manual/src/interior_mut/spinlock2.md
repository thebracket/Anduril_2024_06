# Interior Structure of a Spin Lock

Let's start with the most minimal `SpinLock` we can come up with:

```rust
use std::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering::{Acquire, Release};

pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self { locked: AtomicBool::new(false) }
    }

    pub fn lock(&self) {
        while self.locked.swap(true, Acquire) {
            // This is a CPU hint that a loop is "spinning". On some
            // CPUs it lowers power consumption.
            std::hint::spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

fn main() {
    let my_lock = SpinLock::new();
    my_lock.lock();
    my_lock.unlock();
}
```

Notice that we're using `Acquire` and `Release` in the atomic to guarantee ordered atomic access. Hilariously, the Rust Standard refers you to the C++ standard for atomic ordering!