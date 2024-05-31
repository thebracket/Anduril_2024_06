# Webserver Workshop

Now that you've had some async theory, let's put it into practice.

Start off by setting up a project:

```bash
cargo new webserver
cd webserver
cargo add tokio -F full
```

And setup an async main function that prints "Hello World".

![](../images/ScrollTime.png)

Here's my version:

```rust
#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
```

> Ok, so that one was just to make sure you had a working Tokio setup!