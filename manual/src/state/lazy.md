# Lazy and Once Types

Global variables generally aren't the best of ideas, but sometimes having a "local global" (a static variable inside a module, with controlled access) makes sense.

So this is the most primitive of your options. Sometimes, it fits.

The most common means of setting this up uses a crate called `once_cell` (which is in the process of merging into the standard library). Here's a static variable with full Mutex protection:

```rust
use once_cell::sync::Lazy;
use std::sync::Mutex;

static MY_STATE: Lazy<Mutex<MyType>> = Lazy::new(|| Mutex::new(MyType::default()));

#[derive(Default, Debug)]
struct MyType {
    a: i32,
    b: i32,
}

fn main() {
    let lock = MY_STATE.lock().unwrap();
    println!("{:?}", *lock);
}
```

The `Lazy` means it is initialized the first time you reference it. This gets around the need for a constant function (you can omit the `Lazy` if you can const initialize).

> Don't forget: you can put the Mutex on the inside for finer-grained locking.

Typically, you'd keep the `static` private---and expose some functions to retrieve and/or mutate the global state. That gives you a controled entry-point for state.

> *Be Careful*: You can still make spaghetti if you approach state changes willy nilly!

This can be useful when you have a single state that needs updating. For example, my project `LibreQoS` has a few of these that are tied to tight timers that periodically feed updates (into a channel, which applies the updates) to genuinely global state - the status of a NIC and a copy of an in-kernel map.

> *Don't* just expose a `pub static MY_STATE` and use it everywhere. Make it obvious. There's nothing that ruins a day like tracing through to find out where a state change came from!