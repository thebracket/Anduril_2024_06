use axum::{extract::Path, response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();

    let app = Router::new()
        .route("/", get(say_hello))
        .route("/hello/:n", get(html_path))
        .route("/json/:n", get(json_path));

    axum::serve(listener, app).await.unwrap();
}

async fn say_hello() -> &'static str {
    "Hello, World!"
}

async fn html_path(
    Path(n): Path<u32>,
) -> Html<String> {
    let base = include_str!("hello.html");
    let templated = base.replace("$$MYPICK$$", &n.to_string());
    Html(templated)
}

#[derive(serde::Serialize)]
struct MyData {
    name: String,
    age: u32,
}

async fn json_path(
    Path(n): Path<u32>,
) -> axum::Json<MyData> {
    axum::Json(MyData {
        name: "Alice".to_string(),
        age: n,
    })
}