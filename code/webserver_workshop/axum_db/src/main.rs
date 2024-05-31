use axum::{extract::Path, response::Html, routing::get, Extension, Json, Router};

#[tokio::main]
async fn main() {
    // Run dotenvy
    let _ = dotenvy::dotenv(); // It's ok to not have a .env file
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_pool = sqlx::SqlitePool::connect(&database_url)
        .await.unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();

    let app = Router::new()
        .route("/", get(say_hello))
        .route("/hello/:n", get(html_path))
        .route("/json/:n", get(json_path))
        .route("/person/:id", get(get_person))
        .layer(Extension(connection_pool));

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

#[derive(serde::Serialize, sqlx::FromRow)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}

async fn get_person(
    Path(id): Path<i32>,
    Extension(pool): Extension<sqlx::SqlitePool>,
) -> Json<Person> {
    let person = sqlx::query_as("SELECT * FROM my_data WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();

    Json(person)
}