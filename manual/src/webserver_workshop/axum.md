# Intro to Axum

Axum and Actix are the two most popular webserver systems in the Rust world. Actix is more "actor model" oriented, Axum (made by the same people as Tokio) is a little more traditional. Both can be very similar - if you know one, you can learn the other *really fast*.

Axum sets up a similar model to Express on NodeJS. You have a "router" mapping requests to functions.

Add `axum` to your project:

```bash
cargo add axum
```

And now let's build a "Hello World" webserver:

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Where should we listen on the network?
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();

    // Build a router to map URLs to functions
    let app = Router::new()
        .route("/", get(say_hello));

    axum::serve(listener, app).await.unwrap();
}

// We're returning a static string. There's a LOT of return
// types that all implement `IntoResponse`.
async fn say_hello() -> &'static str {
    "Hello, World!"
}
```

Run this and make sure you get "Hello World" in a browser pointed at [http://127.0.0.1:3001](http://127.0.0.1:3001).