# Async Pitfalls

There's a few pitfalls to watch out for:

* Async traits aren't fully stabilized yet. Use the `async_trait` crate in the meantime.
* Recursively calling async functions is messy. The `async-recursion` crate to help.
* Calling `spawn` requires that what you are spawning be `Sync+Send`. You can use `spawn_local` to ensure that the task spawns on the current thread to avoid this (it won't jump threads).
* Locking a `std::sync::Mutex` in async land can lead to a deadlock, since it uses thread notification! Use `tokio::sync::Mutex` instead. The same is true for `RwLock` and memory barriers.
* Lifetimes get messy in async land, too. It's improving, but it makes ownership hard! VERY often, shared resources will be passed around as `Arc<MyType>` rather than just `MyType`. There's minimal overhead --- it's a `shared_ptr` --- but it avoids the lifetime problems altogether.
