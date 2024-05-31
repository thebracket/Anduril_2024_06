# Code Along: Passing JSON

Axum also provides `JSON` support---both as an extractor and as a response type. So you can receive `JSON` going in, and send `JSON` out.

Let's add a simple JSON extraction as a test. You'll need to add the `serde` crate, with the feature flag `derive`:

```bash
cargo add serde -F derive
```

Serde can derive serialization for a type like this:

```rust
#[derive(serde::Serialize)]
struct MyData {
    name: String,
    age: u32,
}
```

In turn, you can make a handler function that returns a serialized structure pretty easily:

```rust
async fn json_path(
    Path(n): Path<u32>,
) -> axum::Json<MyData> {
    axum::Json(MyData {
        name: "Alice".to_string(),
        age: n,
    })
}
```

Adding it to the router is the same:

```rust
let app = Router::new()
    .route("/", get(say_hello))
    .route("/hello/:n", get(html_path))
    .route("/json/:n", get(json_path));
```

And now run the server, go to [http://localhost:3001/json/5](http://localhost:3001/json/5) and you'll see valid JSON.
