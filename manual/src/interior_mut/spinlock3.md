# Storing Data in the Spin Lock

So far, so good - but we aren't storing anything to protect! Let's add some contents using generics and `UnsafeCell`---which leaves it entirely up to us to provide the protection.

```rust
use std::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering::{Acquire, Release};
use std::cell::UnsafeCell;

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>
}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self { 
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
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

// Since `UnsafeCell` isn't `Sync` (for safety reasons), but we're providing a safe 
//wrapper---we're going to use a bit of `unsafe` code for its intended purpose.

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

fn main() {
    let my_lock = SpinLock::new(12);
    my_lock.lock();
    my_lock.unlock();
}
```
