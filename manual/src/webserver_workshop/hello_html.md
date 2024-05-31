# Code Along: Setup a Hello World Server

It's boring to just return static text (and not very useful).

Add a file to your project (in the `src` directory) named `hello.html`:

```html
<html>
<head>
    <title>Hello</title>
</head>
<body>
    <h1>Hello World</h1>
    You picked $$MYPICK$$
</body>
</html>
```

There are static file serving mechanisms, but we'll write a little retriever. See how we added a template field (we're not going to actually use a templating engine; Moustache is the most popular if you want to)? We're going to replace that on the server side.

Create a new function:

```rust
async fn html_path(
    Path(n): Path<u32>,
) -> Html<String> {
    let base = include_str!("hello.html");
    let templated = base.replace("$$MYPICK$$", &n.to_string());
    Html(templated)
}
```

* We're using `include_str!` to embed the `hello.html` file directly into our file.
* We use a single string replace to substitue `n` for the placeholder.

And now we get to a new concept. Axum supports "extractors". Extractors allow Axum to inject dependencies or data (there's some remarkable template magic to make this work). So in this case:

* The left hand side is destructuring. The structure `Path` contains a tuple.
* The right hand side specifies type type: `Path<u32>`.

So now in the router, we add:

```rust
let app = Router::new()
    .route("/", get(say_hello))
    .route("/hello/:n", get(html_path));
```

Axum will find the `:n` in the URL, and if it converts to a `u32` the `html_path` function will be called---with `n` filled in from the URL.

So now if you go to [http://localhost:3001/hello/5](http://localhost:3001/hello/5) (with the server running), you'll see "You picked 5". If you pick something that isn't convertible to a `u32`, you get a parse error message.