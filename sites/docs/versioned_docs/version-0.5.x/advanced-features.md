---
sidebar_position: 10
---

# Advanced Features

Ngyn provides several advanced features that allow you to build complex, high-performance web applications. This guide explores these features and shows you how to leverage them in your projects.

## Dependency Injection

Ngyn supports a simple form of dependency injection through its handler system, allowing you to inject services and other dependencies into your route handlers.

### Creating Services

You can create services that encapsulate business logic and inject them into your handlers:

```rust
use ngyn::prelude::*;

// Define a service
struct UserService {
    // You might have a database connection or other dependencies here
}

impl UserService {
    fn new() -> Self {
        Self {}
    }
    
    fn get_user(&self, id: &str) -> Result<String, String> {
        // In a real application, you would fetch from a database
        if id == "1" {
            Ok("John Doe".to_string())
        } else {
            Err(format!("User with ID {} not found", id))
        }
    }
}

// Use the service in a handler
#[handler]
fn get_user(param: Param, service: UserService) -> Result<String, String> {
    let user_id = param.get("id").unwrap_or("0");
    service.get_user(user_id)
}

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();
    
    // Create the service
    let user_service = UserService::new();
    
    // Register the route with the service
    app.get("/users/{id}", get_user.with(user_service));
    
    let _ = app.listen("127.0.0.1:3000").await;
}
```

## Custom Transducers

Transducers in Ngyn convert between different data types. You can create custom transducers to handle specific data conversion needs:

```rust
use ngyn::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

// Implement the Transducer trait for your custom type
impl Transducer for User {
    fn transduce(self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        // Convert the User to JSON bytes
        let json = serde_json::to_vec(&self)?;
        Ok(json)
    }
}

#[handler]
fn get_user() -> User {
    User {
        id: 1,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    }
}
```

## Custom Middleware Chains

You can create complex middleware chains to process requests in a specific order:

```rust
use ngyn::prelude::*;

struct LoggerMiddleware;
struct AuthMiddleware;
struct RateLimiterMiddleware;

impl NgynMiddleware for LoggerMiddleware {
    async fn handle(ctx: NgynContext) {
        println!("Request: {} {}", ctx.request().method(), ctx.request().uri());
    }
}

impl NgynMiddleware for AuthMiddleware {
    async fn handle(ctx: NgynContext) {
        let auth_header = ctx.request().headers().get("Authorization");
        if auth_header.is_none() {
            *ctx.response_mut().status_mut() = http::StatusCode::UNAUTHORIZED;
        }
    }
}

impl NgynMiddleware for RateLimiterMiddleware {
    async fn handle(ctx: NgynContext) {
        // Implement rate limiting logic here
        // For example, check if the client has exceeded the request limit
        // If so, set the response status to 429 Too Many Requests
    }
}

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();
    
    // Add global middleware (applied to all routes)
    app.use_middleware(LoggerMiddleware {});
    
    // Create a group with specific middleware
    app.group("/api", |group| {
        // Add middleware specific to this group
        group.use_middleware(AuthMiddleware {});
        group.use_middleware(RateLimiterMiddleware {});
        
        // Define routes within this group
        group.get("/users", get_users);
        group.post("/users", create_user);
    });
    
    let _ = app.listen("127.0.0.1:3000").await;
}
```

## Static File Serving

Ngyn can serve static files from a directory:

```rust
use std::path::PathBuf;
use ngyn::prelude::*;

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();
    
    // Serve static files from the "public" directory
    let _ = app.use_static(PathBuf::from("public"));
    
    // Your other routes
    app.get("/api/hello", handler(|_| "Hello, World!"));
    
    let _ = app.listen("127.0.0.1:3000").await;
}
```

## Custom Error Handling

You can implement custom error handling to provide better error responses:

```rust
use ngyn::prelude::*;
use std::fmt;

// Define a custom error type
#[derive(Debug)]
enum AppError {
    NotFound(String),
    Unauthorized,
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(resource) => write!(f, "{} not found", resource),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

// Implement conversion from AppError to NgynResponse
impl From<AppError> for NgynResponse {
    fn from(error: AppError) -> Self {
        let (status, message) = match error {
            AppError::NotFound(_) => (http::StatusCode::NOT_FOUND, error.to_string()),
            AppError::Unauthorized => (http::StatusCode::UNAUTHORIZED, error.to_string()),
            AppError::InternalError(_) => (http::StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
        };
        
        let mut response = NgynResponse::new(Body::from(message));
        *response.status_mut() = status;
        response
    }
}

// Use the custom error in a handler
#[handler]
fn get_user(param: Param) -> Result<String, AppError> {
    let user_id = param.get("id").unwrap_or("0");
    
    if user_id == "0" {
        return Err(AppError::NotFound("User".to_string()));
    }
    
    if !is_authorized() {
        return Err(AppError::Unauthorized);
    }
    
    match get_user_from_database(user_id) {
        Ok(user) => Ok(user),
        Err(e) => Err(AppError::InternalError(e.to_string())),
    }
}
```

## Async Database Connections

Ngyn works well with async database libraries like `sqlx`:

```rust
use ngyn::prelude::*;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

#[derive(sqlx::FromRow, serde::Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[handler]
async fn get_users(db: Pool<Postgres>) -> Result<JsonResult, String> {
    match sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(&db)
        .await
    {
        Ok(users) => Ok(Ok(json!({ "users": users }))),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://username:password@localhost/database")
        .await?
    
    let mut app = HyperApplication::default();
    
    // Register the route with the database pool
    app.get("/users", get_users.with(pool.clone()));
    
    let _ = app.listen("127.0.0.1:3000").await;
    
    Ok(())
}
```

## GraphQL Integration

Ngyn can be integrated with GraphQL libraries like `async-graphql`:

```rust
use ngyn::prelude::*;
use async_graphql::{Schema, EmptyMutation, EmptySubscription, Object, SimpleObject};
use async_graphql_hyper::GraphQLRequest;

#[derive(SimpleObject)]
struct User {
    id: i32,
    name: String,
    email: String,
}

struct Query;

#[Object]
impl Query {
    async fn users(&self) -> Vec<User> {
        // In a real application, you would fetch from a database
        vec![User {
            id: 1,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        }]
    }
    
    async fn user(&self, id: i32) -> Option<User> {
        if id == 1 {
            Some(User {
                id: 1,
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
            })
        } else {
            None
        }
    }
}

type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[handler]
async fn graphql_handler(schema: MySchema, req: NgynRequest, body: Body) -> Result<JsonResult, String> {
    let query = match body.json::<GraphQLRequest>().await {
        Ok(query) => query,
        Err(e) => return Err(format!("Invalid GraphQL request: {}", e)),
    };
    
    let response = query.execute(&schema).await;
    Ok(Ok(serde_json::to_value(response)?))
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();
    
    let mut app = HyperApplication::default();
    
    // Register the GraphQL endpoint
    app.post("/graphql", graphql_handler.with(schema.clone()));
    
    let _ = app.listen("127.0.0.1:3000").await;
}
```

## Performance Optimization

Here are some tips for optimizing the performance of your Ngyn application:

### Connection Pooling

Use connection pooling for database connections to avoid the overhead of creating new connections for each request:

```rust
let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://username:password@localhost/database")
    .await?
```

### Response Caching

Implement response caching for frequently accessed resources:

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

struct CacheEntry {
    data: Vec<u8>,
    expires_at: Instant,
}

struct CacheMiddleware {
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
    ttl: Duration,
}

impl CacheMiddleware {
    fn new(ttl: Duration) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            ttl,
        }
    }
}

impl NgynMiddleware for CacheMiddleware {
    async fn handle(ctx: NgynContext) {
        let path = ctx.request().uri().path().to_string();
        
        // Check if the response is cached
        let cached_response = {
            let cache = self.cache.lock().unwrap();
            cache.get(&path).and_then(|entry| {
                if entry.expires_at > Instant::now() {
                    Some(entry.data.clone())
                } else {
                    None
                }
            })
        };
        
        if let Some(data) = cached_response {
            // Return the cached response
            *ctx.response_mut() = NgynResponse::new(Body::from(data));
        } else {
            // Process the request normally
            // After the response is generated, cache it
            let response_data = ctx.response().body().to_bytes().await.unwrap().to_vec();
            
            let mut cache = self.cache.lock().unwrap();
            cache.insert(path, CacheEntry {
                data: response_data,
                expires_at: Instant::now() + self.ttl,
            });
        }
    }
}
```

### Asynchronous Processing

Use asynchronous processing for CPU-intensive tasks to avoid blocking the event loop:

```rust
#[handler]
async fn process_data(body: Body) -> Result<String, String> {
    let data = body.text().await?;
    
    // Spawn a blocking task for CPU-intensive processing
    let result = tokio::task::spawn_blocking(move || {
        // Perform CPU-intensive processing here
        // For example, parsing a large JSON file or performing complex calculations
        process_data_intensively(&data)
    }).await.map_err(|e| format!("Task failed: {}", e))??;
    
    Ok(result)
}
```


For more advanced examples, check out the [examples](https://github.com/ngyn-rs/ngyn/tree/main/examples) in the Ngyn repository.