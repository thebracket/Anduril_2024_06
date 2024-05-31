# Let's Add a Cache Layer

So you saw the apparant "magic" of being able to add a dependency as a layer, and have it added to your methods whenever you need them.

This works by storing the layer object, and if a matching *type* is requested (use a new class wrapping connections if you have lots of databases!) the stored data calls `clone()` and sends you a clone.

The use of `clone()` is deliberate. `Arc` is designed for just this: it's a shared pointer, so cloning it gives you the original data - and increases the reference count (which decreases when the `Arc` is dropped).

Given:

* You have a `Person` structure (hint: but probably want to add `Clone` to it).
* You have the code to make an SQL query.
* You know how endpoints work.

Make a structure named `PersonCache`: 
* Store a map in there, indexed by primary key and containing the `Person` record.
* Add a "get" method that returns a cloned Person record if one is stored, None if not.
* Add a "store" method that adds a person to the map.

Now create a cache instance with `Arc::new(PersonCache)` and add it as a layer.

Adjust your `get_person` function to first check the cache. If it looks up a person from the database and finds them, add them to the cache for next time. (Hint: you're going to also need a locking semantic. Use an async one.)

![](../images/ScrollTime.png)

Here's my version:

```rust
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
```