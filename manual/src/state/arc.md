# Arc for Shared Resources

Next up the list in terms of simplicity is the good old `Arc`. Atomic reference counting is *fast*, and is sometimes referred to by Rust teachers as "opting out of lifetimes": you aren't tracking ownership, and the `Arc` won't go away as long as an instance still has references (via `clone()` - not `&`).

We used `Arc` quite often. In the webserver demo, `axum` used it to store database pool handles, we used it to store a site-wide cache. If you're using one of the *many* server systems that uses `Tower` middleware (including Axum and Actix for webservers, Tonic for gRPC) this is built in: you can store a `Layer` or `Extension` containing an `Arc` and have it injected into any handler.

You can use this quite easily:

```rust
#[derive(Default)]
struct MySharedResource {
    data: i32,
}

fn main() {
    let data = Arc::new(MySharedResource::default())

    // Spawn everything that needs access to data
    // start_system(data.clone());
    // etc.
}
```

It's not uncommon to replace some of the "state" handles with passing `Arc` around (much as you'd use a `shared_ptr` in C++). You can even combine it with a `Lazy` setup to have a single point for acquiring state, if you'd like to reduce the weight of your functions listing shared state!
