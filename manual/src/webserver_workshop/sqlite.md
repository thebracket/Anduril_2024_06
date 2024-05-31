# Code Along: An SQLite Database with SQLX

I didn't want to ask you to install a full database server for a workshop, so we'll use `sqlite`. 

You're going to want to use SQLx's command line tool in addition to the library. You'll also use a crate called `dotenvy` that handles environment variables, and loading them from `.env` files.

```bash
cargo install sqlx-cli
cargo add sqlx -F runtime-tokio-rustls -F sqlite
cargo add dotenvy
```

## Make a Database

Create a file (next to `Cargo.toml`) named `.env`:

```
DATABASE_URL="sqlite://my_database.db"
```

> `sqlx-cli` uses your .env file, too!

Now you can make the database:

```bash
sqlx database create
```

You'll see that a "migrations" directory has appeared, and `my_database.db` now exists.

Migrations are a way to store steps to bring your database up to a known state. A hidden migrations table stores which ones have run, so migrations won't run twice.

Create a new migration:

```bash
sqlx migrate add initial
```

A timestamped SQL file has appeared in your `migrations/` directory. It contains a helpful `--Add migration script here` comment. Let's put some actual SQL in there:

```sql
CREATE TABLE my_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    age INTEGER NOT NULL
);

INSERT INTO my_data (name, age) VALUES ('Alice', 42);
INSERT INTO my_data (name, age) VALUES ('Bob', 69);
```

We can then run the migration with `sqlx migrate run`.

## Read From the Database

Open your app. The first thing we're going to do is read the `.env` file to get the `DATABASE_URL`. At the top of your `main` function:

```rust
// Run dotenvy
let _ = dotenvy::dotenv(); // It's ok to not have a .env file
let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
```

Now you want to create an SQLX connection pool:

```rust
let connection_pool = sqlx::SqlitePool::connect(&database_url)
    .await.unwrap();
```

So now we have a database connection pool. But we want it to be available to methods. Axum has "layers" for this. Add a line to the router:

```rust
let app = Router::new()
    .route("/", get(say_hello))
    .route("/hello/:n", get(html_path))
    .route("/json/:n", get(json_path))
    .layer(Extension(connection_pool));
```

Now let's make a structure to represent our table:

```rust
#[derive(serde::Serialize, sqlx::FromRow)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}
```

```rust
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
```

Finally, we add that to the router:

```rust
.route("/person/:id", get(get_person))
```

And now we can go to [http://localhost:3001/person/1](http://localhost:3001/person/1) and have a working JSON retrieval system.

I'm not going to bore you all senseless by having you enter all of the **C**reate, **R**ead, **U**date, **Delete** functions. You can substitute `get()` with `post()` and the other HTTP verbs. You can include `data: Json<MyType>` on posted data to automatically deserialize incoming JSON. It's very powerful, and very productive.