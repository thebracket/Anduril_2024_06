# Accessing Data

Safely storing locked data and not accessing it isn't very useful. Having explicit lock and unlock methods isn't very Rustacean, or convenient---we don't want to return to forgetting to unlock, nor do we want to allow access to the contents without acquiring a lock.

So we need to write our own lock guard.

```rust
use std::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering::{Acquire, Release};
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

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

    // Changed to return a Guard
    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        Guard { lock: self }
    }
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

// The Guard

// We need a lifetime - Rust lifetime elision doesn't work here.
// The compiler error message tells you exactly what to add!
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

// Implementing `Drop` means that when the lock guard goes
// out of scope, it unlocks the SpinLock. We've moved the
// unlock function into here.
impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}

// Implementing `Deref` allows you to access the contents
// transparently, like other locks.
// The "Safety" comment is required by Clippy to explain
// unsafe code blocks. I've used Mara's comment.
impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // Safety: The very existence of this Guard
        // guarantees we've exclusively locked the lock.
        unsafe { &*self.lock.value.get() }
    }
}

// `DerefMut` is the same - but for mutable access.
impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: The very existence of this Guard
        // guarantees we've exclusively locked the lock.
        unsafe { &mut *self.lock.value.get() }
    }
}

// If T is Send, then the Guard can be Send.
unsafe impl<T> Send for Guard<'_, T> where T: Send {}
// If T is Sync, then the Guard can be Sync.
unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}

// Quick test of the lock
fn main() {
    let my_lock = SpinLock::new(12);
    {
        let lock = my_lock.lock();
        println!("{}", *lock);
    }

    {
        let mut lock = my_lock.lock();
        *lock = 14;
        println!("{}", *lock);
    }
}
```