use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();

    let app = Router::new()
        .route("/", get(say_hello));

    axum::serve(listener, app).await.unwrap();
}

async fn say_hello() -> &'static str {
    "Hello, World!"
}