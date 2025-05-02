---
sidebar_position: 11
---

# Database Integration

Integrating a database with your Ngyn application is essential for building applications that need to persist data. This guide will show you how to connect your Ngyn application to various databases and perform common operations.

## Supported Database Libraries

Ngyn works well with various Rust database libraries. Here are some popular options:

-   **SQLx**: Type-safe SQL for Rust with compile-time checked queries
-   **Diesel**: A safe, extensible ORM and query builder
-   **Tokio Postgres**: An async PostgreSQL client
-   **MongoDB**: Official MongoDB driver for Rust
-   **Redis**: Redis client for Rust

## Setting Up SQLx with PostgreSQL

SQLx is a popular choice for Rust applications due to its async support and compile-time query checking. Here's how to set it up with Ngyn, based on the official example in the Ngyn repository:

### 1. Add Dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
ngyn = "0.5"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "time"] }
dotenv = "0.15.0"
```

### 2. Set Up the Database Connection

Ngyn makes it easy to integrate database connections using the `AppState` pattern. Here's how to set up a simple SQLx connection with PostgreSQL:

```rust
use ngyn::prelude::*;
use sqlx::{Connection, PgConnection};

// Define your application state to hold the database connection
#[derive(AppState)]
struct State {
    conn: PgConnection,
}

// Define a parameter struct for route parameters
#[derive(Param)]
struct HandleParam {
    id: i32,
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create the application
    let mut app = HyperApplication::default();

    // Connect to the database
    let conn = PgConnection::connect(&database_url).await.unwrap();

    // Set the application state with the database connection
    app.set_state(State { conn });

    // Register routes
    app.get("/{id}", async_wrap(handle_get));

    println!("Starting server at http://127.0.0.1:8080");
    let _ = app.listen("0.0.0.0:8080").await;
}
```

### 3. Implement Route Handlers

With the state and parameter structs defined, you can now implement your route handlers. Here's a simple example that queries a PostgreSQL database:

```rust
#[handler]
async fn handle_get(param: HandleParam, state: &mut State) -> String {
    match sqlx::query!("SELECT * FROM users WHERE id = $1", param.id)
        .fetch_one(&mut state.conn)
        .await
    {
        Ok(record) => record.name.unwrap(),
        Err(_) => "Not found".to_string(),
    }
}
```

This handler:

1. Receives the route parameter (`id`) through the `HandleParam` struct
2. Accesses the database connection from the application state
3. Executes a SQL query with the `sqlx::query!` macro, which provides compile-time checking
4. Returns the user's name as a string if found, or "Not found" otherwise

### 4. More Advanced Query Examples

For more complex scenarios, you might want to use structured data models:

```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[handler]
async fn get_user(param: HandleParam, state: &mut State) -> Result<JsonResult, String> {
    match sqlx::query_as!(User, "SELECT id, name, email FROM users WHERE id = $1", param.id)
        .fetch_optional(&mut state.conn)
        .await
    {
        Ok(Some(user)) => Ok(Ok(json!(user))),
        Ok(None) => Err(format!("User with ID {} not found", param.id)),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}
```

## Using Diesel ORM

Diesel is a powerful ORM for Rust that provides type-safe SQL. Here's how to use it with Ngyn:

### 1. Add Dependencies

```toml
[dependencies]
ngyn = "0.5"
tokio = { version = "1", features = ["full"] }
diesel = { version = "2.1", features = ["postgres", "r2d2", "chrono"] }
r2d2 = "0.8"
serde = { version = "1", features = ["derive"] }
```

### 2. Set Up the Database Connection

```rust
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use ngyn::prelude::*;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Define your application state to hold the database connection pool
#[derive(AppState)]
struct State {
    pool: DbPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the database connection pool
    let manager = ConnectionManager::<PgConnection>::new("postgres://username:password@localhost/database");
    let pool = r2d2::Pool::builder()
        .max_size(5)
        .build(manager)?;

    let mut app = HyperApplication::default();

    // Set the application state with the database connection pool
    app.set_state(State { pool });

    // Register routes
    app.get("/users", async_wrap(get_users));
    app.get("/users/{id}", async_wrap(get_user));
    app.post("/users", async_wrap(create_user));

    println!("Server running at http://127.0.0.1:3000");
    let _ = app.listen("127.0.0.1:3000").await;

    Ok(())
}
```

### 3. Define Schema and Models

```rust
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

#[derive(Queryable, Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
    created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
struct NewUser {
    name: String,
    email: String,
    created_at: chrono::NaiveDateTime,
}
```

### 4. Implement Route Handlers

```rust
use diesel::RunQueryDsl;

#[handler]
async fn get_users(state: &mut State) -> Result<JsonResult, String> {
    // Use tokio::task::spawn_blocking for database operations
    let pool = state.pool.clone();
    let users = tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|e| format!("Connection error: {}", e))?;
        users::table
            .order_by(users::created_at.desc())
            .load::<User>(&conn)
            .map_err(|e| format!("Database error: {}", e))
    })
    .await
    .map_err(|e| format!("Task error: {}", e))??;

    Ok(Ok(json!({ "users": users })))
}

#[handler]
async fn get_user(param: Param, state: &mut State) -> Result<JsonResult, String> {
    let user_id = param.get("id")
        .unwrap_or("0")
        .parse::<i32>()
        .map_err(|_| "Invalid user ID".to_string())?;

    let pool = state.pool.clone();
    let user = tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|e| format!("Connection error: {}", e))?;
        users::table
            .find(user_id)
            .first::<User>(&conn)
            .optional()
            .map_err(|e| format!("Database error: {}", e))
    })
    .await
    .map_err(|e| format!("Task error: {}", e))??;

    match user {
        Some(user) => Ok(Ok(json!(user))),
        None => Err(format!("User with ID {} not found", user_id)),
    }
}

#[handler]
async fn create_user(body: Body, state: &mut State) -> Result<JsonResult, String> {
    let user_req = match body.json::<NewUser>().await {
        Ok(mut req) => {
            req.created_at = chrono::Utc::now().naive_utc();
            req
        },
        Err(e) => return Err(format!("Invalid request body: {}", e)),
    };

    let pool = state.pool.clone();
    let user = tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|e| format!("Connection error: {}", e))?;
        diesel::insert_into(users::table)
            .values(&user_req)
            .get_result::<User>(&conn)
            .map_err(|e| format!("Failed to create user: {}", e))
    })
    .await
    .map_err(|e| format!("Task error: {}", e))??;

    Ok(Ok(json!({
        "message": "User created successfully",
        "user": user
    })))
}
```

## MongoDB Integration

For NoSQL databases, MongoDB is a popular choice. Here's how to integrate it with Ngyn:

### 1. Add Dependencies

```toml
[dependencies]
ngyn = "0.5"
tokio = { version = "1", features = ["full"] }
mongodb = "2.6"
serde = { version = "1", features = ["derive"] }
futures = "0.3"
```

### 2. Set Up the MongoDB Connection

```rust
use mongodb::{Client, options::ClientOptions, Collection};
use ngyn::prelude::*;

// Define your application state to hold the MongoDB collection
#[derive(AppState)]
struct State {
    users_collection: Collection<User>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up MongoDB client
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("mydb");
    let users_collection = db.collection::<User>("users");

    let mut app = HyperApplication::default();

    // Set the application state with the MongoDB collection
    app.set_state(State { users_collection });

    // Register routes
    app.get("/users", async_wrap(get_users));
    app.get("/users/{id}", async_wrap(get_user));
    app.post("/users", async_wrap(create_user));

    println!("Server running at http://127.0.0.1:3000");
    let _ = app.listen("127.0.0.1:3000").await;

    Ok(())
}
```

### 3. Define Data Models

```rust
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    email: String,
    created_at: DateTime,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}
```

### 4. Implement Route Handlers

```rust
use futures::stream::TryStreamExt;
use mongodb::{bson::{doc, oid::ObjectId}, Collection};

#[handler]
async fn get_users(state: &mut State) -> Result<JsonResult, String> {
    let mut cursor = state.users_collection.find(None, None)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    let mut users = Vec::new();
    while let Some(user) = cursor.try_next().await.map_err(|e| format!("Cursor error: {}", e))? {
        users.push(user);
    }

    Ok(Ok(json!({ "users": users })))
}

#[handler]
async fn get_user(param: Param, state: &mut State) -> Result<JsonResult, String> {
    let id = param.get("id").unwrap_or("");
    let object_id = ObjectId::parse_str(id).map_err(|_| "Invalid ID format".to_string())?;

    let filter = doc! { "_id": object_id };
    let user = state.users_collection.find_one(filter, None)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    match user {
        Some(user) => Ok(Ok(json!(user))),
        None => Err(format!("User with ID {} not found", id)),
    }
}

#[handler]
async fn create_user(body: Body, state: &mut State) -> Result<JsonResult, String> {
    let user_req = match body.json::<CreateUserRequest>().await {
        Ok(req) => req,
        Err(e) => return Err(format!("Invalid request body: {}", e)),
    };

    let user = User {
        id: None,
        name: user_req.name,
        email: user_req.email,
        created_at: mongodb::bson::DateTime::now(),
    };

    let result = state.users_collection.insert_one(user, None)
        .await
        .map_err(|e| format!("Failed to create user: {}", e))?;

    let inserted_id = result.inserted_id.as_object_id()
        .ok_or_else(|| "Failed to get inserted ID".to_string())?;

    let filter = doc! { "_id": inserted_id };
    let created_user = state.users_collection.find_one(filter, None)
        .await
        .map_err(|e| format!("Failed to fetch created user: {}", e))?;

    Ok(Ok(json!({
        "message": "User created successfully",
        "user": created_user
    })))
}
```

## Redis Integration

Redis is useful for caching, session storage, and other scenarios where fast access to data is required:

### 1. Add Dependencies

```toml
[dependencies]
ngyn = "0.5"
tokio = { version = "1", features = ["full"] }
redis = { version = "0.23", features = ["tokio-comp"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
```

### 2. Set Up the Redis Connection

```rust
use ngyn::prelude::*;
use redis::{Client, AsyncCommands};

// Define your application state to hold the Redis client
#[derive(AppState)]
struct State {
    client: redis::Client,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up Redis client
    let client = redis::Client::open("redis://127.0.0.1/")?;

    let mut app = HyperApplication::default();

    // Set the application state with the Redis client
    app.set_state(State { client });

    // Register routes
    app.get("/cache/{key}", async_wrap(get_cached_value));
    app.post("/cache/{key}", async_wrap(set_cached_value));

    println!("Server running at http://127.0.0.1:3000");
    let _ = app.listen("127.0.0.1:3000").await;

    Ok(())
}
```

### 3. Implement Route Handlers

```rust
#[handler]
async fn get_cached_value(param: Param, state: &mut State) -> Result<String, String> {
    let key = param.get("key").unwrap_or("");
    if key.is_empty() {
        return Err("Key cannot be empty".to_string());
    }

    let mut conn = state.client.get_async_connection()
        .await
        .map_err(|e| format!("Redis connection error: {}", e))?;

    let value: Option<String> = conn.get(key)
        .await
        .map_err(|e| format!("Redis error: {}", e))?;

    match value {
        Some(val) => Ok(val),
        None => Err(format!("No value found for key: {}", key)),
    }
}

#[handler]
async fn set_cached_value(param: Param, body: Body, state: &mut State) -> Result<String, String> {
    let key = param.get("key").unwrap_or("");
    if key.is_empty() {
        return Err("Key cannot be empty".to_string());
    }

    let value = body.text().await.map_err(|e| format!("Failed to read body: {}", e))?;

    let mut conn = state.client.get_async_connection()
        .await
        .map_err(|e| format!("Redis connection error: {}", e))?;

    let _: () = conn.set(key, value)
        .await
        .map_err(|e| format!("Redis error: {}", e))?;

    Ok(format!("Value for key '{}' set successfully", key))
}
```

## Database Migrations

For SQL databases, it's important to manage schema changes through migrations. Here's how to set up migrations with SQLx:

### 1. Install the SQLx CLI

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### 2. Initialize Migrations

```bash
sqlx migrate add create_users_table
```

This will create a new migration file in the `migrations` directory.

### 3. Write the Migration

Edit the generated migration file to create your table:

```sql
-- migrations/20230101000000_create_users_table.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
```

### 4. Run Migrations in Your Application

```rust
use sqlx::{postgres::PgPoolOptions, migrate::Migrator, Pool, Postgres};
use std::path::Path;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://username:password@localhost/database")
        .await?;

    // Run migrations
    sqlx::migrate!().run(&pool).await?

    // Or load migrations from a directory
    let migrator = Migrator::new(Path::new("./migrations")).await?
    migrator.run(&pool).await?

    // Rest of your application setup
    // ...

    Ok(())
}
```

## Best Practices

### Connection Pooling

Always use connection pooling to avoid the overhead of creating new connections for each request:

```rust
let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://username:password@localhost/database")
    .await?
```

### Environment Variables

As shown in the example above, it's a best practice to store database connection strings in environment variables. The `dotenv` crate makes this easy:

```rust
// Load environment variables from .env file
dotenv::dotenv().ok();
let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

// Connect to the database using the URL from environment variables
let conn = PgConnection::connect(&database_url).await.unwrap();
```

Create a `.env` file in your project root with your database connection string:

```toml
DATABASE_URL=postgres://username:password@localhost/database
```

### Error Handling

Ngyn makes error handling straightforward. You can use Rust's `Result` type to handle database errors in your handlers:

```rust
// Simple error handling with String error messages
#[handler]
async fn get_user(param: HandleParam, state: &mut State) -> Result<String, String> {
    match sqlx::query!("SELECT name FROM users WHERE id = $1", param.id)
        .fetch_optional(&mut state.conn)
        .await
    {
        Ok(Some(record)) => Ok(record.name.unwrap_or_default()),
        Ok(None) => Err(format!("User with ID {} not found", param.id)),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

// For JSON responses
#[handler]
async fn get_user_json(param: HandleParam, state: &mut State) -> Result<JsonResult, String> {
    match sqlx::query_as!(User, "SELECT id, name, email FROM users WHERE id = $1", param.id)
        .fetch_optional(&mut state.conn)
        .await
    {
        Ok(Some(user)) => Ok(Ok(json!(user))),
        Ok(None) => Err(format!("User with ID {} not found", param.id)),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}
```

For more complex applications, you can define custom error types:

```rust
#[derive(Debug)]
enum AppError {
    NotFound(String),
    DatabaseError(String),
    ValidationError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

// Convert SQLx errors to your application errors
impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            _ => AppError::DatabaseError(error.to_string()),
        }
    }
}
```

### Transactions

Use transactions for operations that need to be atomic:

```rust
// Define your application state to hold the database connection pool
#[derive(AppState)]
struct State {
    pool: Pool<Postgres>,
}

#[handler]
async fn transfer_funds(body: Body, state: &mut State) -> Result<JsonResult, String> {
    let transfer = match body.json::<TransferRequest>().await {
        Ok(req) => req,
        Err(e) => return Err(format!("Invalid request body: {}", e)),
    };

    let mut tx = state.pool.begin().await
        .map_err(|e| format!("Failed to start transaction: {}", e))?;

    // Deduct from source account
    let rows_affected = sqlx::query(
        "UPDATE accounts SET balance = balance - $1 WHERE id = $2 AND balance >= $1"
    )
    .bind(transfer.amount)
    .bind(transfer.from_account)
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Database error: {}", e))?
    .rows_affected();

    if rows_affected == 0 {
        tx.rollback().await
            .map_err(|e| format!("Failed to rollback transaction: {}", e))?;
        return Err("Insufficient funds or account not found".to_string());
    }

    // Add to destination account
    sqlx::query(
        "UPDATE accounts SET balance = balance + $1 WHERE id = $2"
    )
    .bind(transfer.amount)
    .bind(transfer.to_account)
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    // Commit the transaction
    tx.commit().await
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    Ok(Ok(json!({
        "message": "Transfer completed successfully"
    })))
}
```

For more advanced database integration examples, check out the [examples](https://github.com/ngyn-rs/ngyn/tree/main/examples) in the Ngyn repository.
