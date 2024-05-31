use std::{collections::HashMap, sync::Arc};

use axum::{extract::Path, routing::get, Extension, Json, Router};

#[tokio::main]
async fn main() {
    // Run dotenvy
    let _ = dotenvy::dotenv(); // It's ok to not have a .env file
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_pool = sqlx::SqlitePool::connect(&database_url)
        .await.unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();

    let app = Router::new()
        .route("/person/:id", get(get_person))
        .layer(Extension(connection_pool))
        .layer(Extension(PersonCache::new()));

    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Serialize, sqlx::FromRow, Clone)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}

struct PersonCache {
    cache: tokio::sync::Mutex<HashMap<i32, Person>>,
}

impl PersonCache {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            cache: tokio::sync::Mutex::new(HashMap::new()),
        })
    }

    async fn add(&self, person: Person) {
        let mut cache = self.cache.lock().await;
        cache.insert(person.id, person);
    }

    async fn get(&self, id: i32) -> Option<Person> {
        let cache = self.cache.lock().await;
        cache.get(&id).cloned()
    }
}

async fn get_person(
    Path(id): Path<i32>,
    Extension(pool): Extension<sqlx::SqlitePool>,
    Extension(cache): Extension<Arc<PersonCache>>,
) -> Json<Person> {
    if let Some(person) = cache.get(id).await {
        println!("Cache Hit");
        return Json(person);
    } else {
        println!("Cache Miss");
        let person: Person = sqlx::query_as("SELECT * FROM my_data WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();

        cache.add(person.clone()).await;
        Json(person)
    }
}