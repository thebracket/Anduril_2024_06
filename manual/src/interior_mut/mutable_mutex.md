# Why Isn't the Mutex Mutable?

Internally, Rust is maintaining a compile-time mutex for everyingthing marked as `mut`. The `mut` keyword only kinda means "mutable" - it really means "the mutex needs to be locked from the outside".

Rust auto-implements two traits:

* `Send` - can a variable safely be transferred *immutably* across thread boundaries? Almost everything can (`Rc` being the obvious exception; it's explicitly NOT `Send`).
* `Sync` - can a variable safely be accessed *immutably* across thread boundaries? Except for read-only constructs, you usually need some form of synchronization for this.

So a mutable integer is `Send`, but isn't `Sync` if anything can write to it - it's not safe to read it if it might be being updated somewhere else.

A `Mutex<i32>` is safe, because the access is *always* immutable --- and `Mutex` provides "traffic light" functionality to ensure that only one write can happen at a time.

`RefCell<i32>` is a special-case, because it promises to act as a safety barrier---and implements the barrier at run-time. It still follows Rust's rules --- so it's sound --- but the burden shifts to the developer. You can use these when you *must* avoid the overhead of a `Mutex`, and can carefully guarantee that writes during read won't happen (the program panics if you try it).
