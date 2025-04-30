---
sidebar_position: 11
---

# Database Integration

Integrating a database with your Ngyn application is essential for building applications that need to persist data. This guide will show you how to connect your Ngyn application to various databases and perform common operations.

## Supported Database Libraries

Ngyn works well with various Rust database libraries. Here are some popular options:

- **SQLx**: Type-safe SQL for Rust with compile-time checked queries
- **Diesel**: A safe, extensible ORM and query builder
- **Tokio Postgres**: An async PostgreSQL client
- **MongoDB**: Official MongoDB driver for Rust
- **Redis**: Redis client for Rust

## Setting Up SQLx with PostgreSQL

SQLx is a popular choice for Rust applications due to its async support and compile-time query checking. Here's how to set it up with Ngyn:

### 1. Add Dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
ngyn = "0.5"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "macros", "json"] }
serde = { version = "1", features = ["derive"] }
```

### 2. Set Up the Database Connection

```rust
use ngyn::prelude::*;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://username:password@localhost/database")
        .await?
    
    let mut app = HyperApplication::default();
    
    // Register routes with the database pool
    app.get("/users", get_users.with(pool.clone()));
    app.get("/users/{id}", get_user.with(pool.clone()));
    app.post("/users", create_user.with(pool.clone()));
    
    println!("Server running at http://127.0.0.1:3000");
    let _ = app.listen("127.0.0.1:3000").await;
    
    Ok(())
}
```

### 3. Define Data Models

```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}
```

### 4. Implement Route Handlers

```rust
#[handler]
async fn get_users(db: Pool<Postgres>) -> Result<JsonResult, String> {
    match sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(&db)
        .await
    {
        Ok(users) => Ok(Ok(json!({ "users": users }))),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

#[handler]
async fn get_user(param: Param, db: Pool<Postgres>) -> Result<JsonResult, String> {
    let user_id = param.get("id")
        .unwrap_or("0")
        .parse::<i32>()
        .map_err(|_| "Invalid user ID".to_string())?;
    
    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&db)
        .await
    {
        Ok(Some(user)) => Ok(Ok(json!(user))),
        Ok(None) => Err(format!("User with ID {} not found", user_id)),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

#[handler]
async fn create_user(body: Body, db: Pool<Postgres>) -> Result<JsonResult, String> {
    let user_req = match body.json::<CreateUserRequest>().await {
        Ok(req) => req,
        Err(e) => return Err(format!("Invalid request body: {}", e)),
    };
    
    match sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, created_at) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&user_req.name)
    .bind(&user_req.email)
    .bind(chrono::Utc::now())
    .fetch_one(&db)
    .await
    {
        Ok(user) => Ok(Ok(json!({
            "message": "User created successfully",
            "user": user
        }))),
        Err(e) => Err(format!("Failed to create user: {}", e)),
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the database connection pool
    let manager = ConnectionManager::<PgConnection>::new("postgres://username:password@localhost/database");
    let pool = r2d2::Pool::builder()
        .max_size(5)
        .build(manager)?
    
    let mut app = HyperApplication::default();
    
    // Register routes with the database pool
    app.get("/users", get_users.with(pool.clone()));
    app.get("/users/{id}", get_user.with(pool.clone()));
    app.post("/users", create_user.with(pool.clone()));
    
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
async fn get_users(db: DbPool) -> Result<JsonResult, String> {
    // Use tokio::task::spawn_blocking for database operations
    let users = tokio::task::spawn_blocking(move || {
        let conn = db.get().map_err(|e| format!("Connection error: {}", e))?;
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
async fn get_user(param: Param, db: DbPool) -> Result<JsonResult, String> {
    let user_id = param.get("id")
        .unwrap_or("0")
        .parse::<i32>()
        .map_err(|_| "Invalid user ID".to_string())?;
    
    let user = tokio::task::spawn_blocking(move || {
        let conn = db.get().map_err(|e| format!("Connection error: {}", e))?;
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
async fn create_user(body: Body, db: DbPool) -> Result<JsonResult, String> {
    let user_req = match body.json::<NewUser>().await {
        Ok(mut req) => {
            req.created_at = chrono::Utc::now().naive_utc();
            req
        },
        Err(e) => return Err(format!("Invalid request body: {}", e)),
    };
    
    let user = tokio::task::spawn_blocking(move || {
        let conn = db.get().map_err(|e| format!("Connection error: {}", e))?;
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
use mongodb::{Client, options::ClientOptions};
use ngyn::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up MongoDB client
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?
    let client = Client::with_options(client_options)?
    let db = client.database("mydb");
    let users_collection = db.collection::<User>("users");
    
    let mut app = HyperApplication::default();
    
    // Register routes with the MongoDB collection
    app.get("/users", get_users.with(users_collection.clone()));
    app.get("/users/{id}", get_user.with(users_collection.clone()));
    app.post("/users", create_user.with(users_collection.clone()));
    
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
async fn get_users(collection: Collection<User>) -> Result<JsonResult, String> {
    let mut cursor = collection.find(None, None)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    let mut users = Vec::new();
    while let Some(user) = cursor.try_next().await.map_err(|e| format!("Cursor error: {}", e))? {
        users.push(user);
    }
    
    Ok(Ok(json!({ "users": users })))
}

#[handler]
async fn get_user(param: Param, collection: Collection<User>) -> Result<JsonResult, String> {
    let id = param.get("id").unwrap_or("");
    let object_id = ObjectId::parse_str(id).map_err(|_| "Invalid ID format".to_string())?;
    
    let filter = doc! { "_id": object_id };
    let user = collection.find_one(filter, None)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    match user {
        Some(user) => Ok(Ok(json!(user))),
        None => Err(format!("User with ID {} not found", id)),
    }
}

#[handler]
async fn create_user(body: Body, collection: Collection<User>) -> Result<JsonResult, String> {
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
    
    let result = collection.insert_one(user, None)
        .await
        .map_err(|e| format!("Failed to create user: {}", e))?;
    
    let inserted_id = result.inserted_id.as_object_id()
        .ok_or_else(|| "Failed to get inserted ID".to_string())?;
    
    let filter = doc! { "_id": inserted_id };
    let created_user = collection.find_one(filter, None)
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up Redis client
    let client = redis::Client::open("redis://127.0.0.1/")?;
    
    let mut app = HyperApplication::default();
    
    // Register routes with the Redis client
    app.get("/cache/{key}", get_cached_value.with(client.clone()));
    app.post("/cache/{key}", set_cached_value.with(client.clone()));
    
    println!("Server running at http://127.0.0.1:3000");
    let _ = app.listen("127.0.0.1:3000").await;
    
    Ok(())
}
```

### 3. Implement Route Handlers

```rust
#[handler]
async fn get_cached_value(param: Param, client: redis::Client) -> Result<String, String> {
    let key = param.get("key").unwrap_or("");
    if key.is_empty() {
        return Err("Key cannot be empty".to_string());
    }
    
    let mut conn = client.get_async_connection()
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
async fn set_cached_value(param: Param, body: Body, client: redis::Client) -> Result<String, String> {
    let key = param.get("key").unwrap_or("");
    if key.is_empty() {
        return Err("Key cannot be empty".to_string());
    }
    
    let value = body.text().await.map_err(|e| format!("Failed to read body: {}", e))?;
    
    let mut conn = client.get_async_connection()
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
        .await?
    
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

Store database connection strings in environment variables:

```rust
use dotenv::dotenv;
use std::env;

dotenv().ok(); // Load .env file if present

let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");

let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await?
```

### Error Handling

Implement proper error handling for database operations:

```rust
#[derive(Debug)]
enum DbError {
    ConnectionError(String),
    QueryError(String),
    NotFound,
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::ConnectionError(msg) => write!(f, "Database connection error: {}", msg),
            DbError::QueryError(msg) => write!(f, "Database query error: {}", msg),
            DbError::NotFound => write!(f, "Resource not found"),
        }
    }
}

impl From<sqlx::Error> for DbError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => DbError::NotFound,
            _ => DbError::QueryError(error.to_string()),
        }
    }
}

// Use in handlers
#[handler]
async fn get_user(param: Param, db: Pool<Postgres>) -> Result<JsonResult, DbError> {
    let user_id = param.get("id")
        .unwrap_or("0")
        .parse::<i32>()
        .map_err(|_| DbError::QueryError("Invalid user ID".to_string()))?;
    
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&db)
        .await?;
    
    match user {
        Some(user) => Ok(Ok(json!(user))),
        None => Err(DbError::NotFound),
    }
}
```

### Transactions

Use transactions for operations that need to be atomic:

```rust
#[handler]
async fn transfer_funds(body: Body, db: Pool<Postgres>) -> Result<JsonResult, String> {
    let transfer = match body.json::<TransferRequest>().await {
        Ok(req) => req,
        Err(e) => return Err(format!("Invalid request body: {}", e)),
    };
    
    let mut tx = db.begin().await
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