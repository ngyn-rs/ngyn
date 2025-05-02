---
sidebar_position: 10
---

# Advanced Features

Ngyn provides a number of advanced features that can help you build more complex applications. This guide covers some of these features and how to use them effectively.

## Dependency Injection

Dependency injection is a design pattern that allows you to inject dependencies into your handlers rather than creating them inside the handler. This makes your code more modular, testable, and maintainable.

Ngyn supports dependency injection through the `State` type, which allows you to access application state from your handlers.

### Basic Dependency Injection

Here's a simple example of dependency injection using the `State` type:

```rust
use ngyn::prelude::*;

// Define a service
struct UserService {
    // Service fields and methods
}

impl UserService {
    fn new() -> Self {
        Self {}
    }

    fn get_user(&self, id: u32) -> Option<String> {
        // In a real application, this would fetch from a database
        if id == 1 {
            Some("John Doe".to_string())
        } else {
            None
        }
    }
}

// Define our application state
#[derive(AppState)]
struct State {
    user_service: UserService,
}

// Handler that uses the service
#[handler]
fn get_user(param: Param, state: State) -> Result<JsonResult, String> {
    let id = param.get("id")
        .unwrap_or("0")
        .parse::<u32>()
        .map_err(|_| "Invalid ID".to_string())?;

    match state.user_service.get_user(id) {
        Some(user) => Ok(Ok(json!({ "user": user }))),
        None => Err(format!("User with ID {} not found", id)),
    }
}

#[tokio::main]
async fn main() {
    // Create the service
    let user_service = UserService::new();

    // Create our application state
    let app_state = State { user_service };

    let mut app = HyperApplication::default();

    // Set the application state
    app.set_state(app_state);

    // Register the route
    app.get("/users/{id}", get_user);

    let _ = app.listen("127.0.0.1:3000").await;
}
```

### Multiple Dependencies

You can inject multiple dependencies by adding them to your application state:

```rust
use ngyn::prelude::*;

#[derive(AppState)]
struct State {
    user_service: UserService,
    post_service: PostService,
    config: AppConfig,
}

#[handler]
fn get_user_posts(param: Param, state: State) -> Result<JsonResult, String> {
    let user_id = param.get("id")
        .unwrap_or("0")
        .parse::<u32>()
        .map_err(|_| "Invalid ID".to_string())?;

    // Use both services
    let user = state.user_service.get_user(user_id)
        .ok_or_else(|| format!("User with ID {} not found", user_id))?;

    let posts = state.post_service.get_posts_by_user(user_id);

    Ok(Ok(json!({
        "user": user,
        "posts": posts,
        "max_posts": state.config.max_posts_per_user
    })))
}
```

## Middleware

Middleware allows you to execute code before and after your handlers. This is useful for tasks like logging, authentication, and error handling.

### Creating Middleware

To create middleware, implement the `NgynMiddleware` trait:

```rust
use ngyn::prelude::*;
use std::time::Instant;

struct TimingMiddleware;

impl NgynMiddleware for TimingMiddleware {
    async fn handle(ctx: NgynContext) {
        let start = Instant::now();

        let duration = start.elapsed();
        println!("Request to {} took {:?}", ctx.request().uri(), duration);
    }
}

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();

    // Apply middleware to all routes
    app.use_middleware(TimingMiddleware {});

    // Define routes
    app.get("/", handler(|_| "Hello, World!"));

    let _ = app.listen("127.0.0.1:3000").await;
}
```

### Middleware for Specific Routes

You can also apply middleware to specific routes or groups of routes:

```rust
// Apply middleware to a specific route
app.get("/admin", handler(|_| "Admin Area"));

// Apply middleware to a group of routes
app.group("/api", |group| {
    group.use_middleware(ApiKeyMiddleware {});

    group.get("/users", get_users);
    group.post("/users", create_user);
});
```

## Gates

Gates are similar to middleware but are specifically designed for authorization. They determine whether a request should proceed to the handler.

### Creating a Gate

To create a gate, implement the `NgynGate` trait:

```rust
use ngyn::prelude::*;

struct AdminGate;

impl NgynGate for AdminGate {
    async fn can_activate(ctx: NgynContext) -> bool {
        // Check if the user is an admin
        // In a real application, this would check a JWT token or session
        let is_admin = ctx.request()
            .headers()
            .get("X-User-Role")
            .and_then(|v| v.to_str().ok())
            .map(|role| role == "admin")
            .unwrap_or(false);

        if !is_admin {
            // Set a 403 Forbidden status
            *ctx.response_mut().status_mut() = http::StatusCode::FORBIDDEN;
        }

        is_admin
    }
}
```

### Using Gates

You can apply gates to handlers using the `gates` attribute:

```rust
#[handler(gates = [AdminGate])]
fn admin_dashboard() -> &'static str {
    "Welcome to the Admin Dashboard"
}

// You can also apply multiple gates
#[handler(gates = [AuthenticatedGate, AdminGate])]
fn super_admin_dashboard() -> &'static str {
    "Welcome to the Super Admin Dashboard"
}
```

## Database Integration

Ngyn works well with various database libraries. Here's an example using SQLx with PostgreSQL:

```rust
use ngyn::prelude::*;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

// Define our application state
#[derive(AppState)]
struct State {
    pool: Pool<Postgres>,
}

#[handler]
async fn get_users(state: State) -> Result<JsonResult, String> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(Ok(json!({ "users": users })))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/mydb")
        .await?;

    // Create our application state
    let app_state = State { pool };

    let mut app = HyperApplication::default();

    // Set the application state
    app.set_state(app_state);

    // Register the route
    app.get("/users", get_users);

    let _ = app.listen("127.0.0.1:3000").await;

    Ok(())
}
```

## GraphQL Integration

Ngyn can be integrated with GraphQL libraries like Juniper:

```rust
use ngyn::prelude::*;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

// Define your GraphQL schema
struct Query;

#[graphql_object]
impl Query {
    fn hello() -> &'static str {
        "Hello, GraphQL!"
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>>;

// Define our application state
#[derive(AppState)]
struct State {
    schema: Schema,
}

#[handler]
async fn graphql_handler(body: Body, state: State) -> Result<JsonResult, String> {
    let request = match body.json::<juniper::http::GraphQLRequest>().await {
        Ok(req) => req,
        Err(e) => return Err(format!("Invalid GraphQL request: {}", e)),
    };

    let response = request.execute(&state.schema, &());
    Ok(Ok(json!(response)))
}

#[tokio::main]
async fn main() {
    // Create the schema
    let schema = Schema::new(Query, EmptyMutation::new(), EmptySubscription::new());

    // Create our application state
    let app_state = State { schema };

    let mut app = HyperApplication::default();

    // Set the application state
    app.set_state(app_state);

    // Register the GraphQL endpoint
    app.post("/graphql", graphql_handler);

    let _ = app.listen("127.0.0.1:3000").await;
}
```

## File Uploads

Ngyn supports file uploads using multipart form data:

```rust
use ngyn::prelude::*;
use futures::TryStreamExt;
use std::io::Write;

#[handler]
async fn upload_handler(body: Body) -> Result<JsonResult, String> {
    let mut uploaded_files = Vec::new();
    let multipart = body.to_multipart()
       .map_err(|e| e.to_string())?;

    let mut multipart = multipart.into_inner();

    while let Ok(Some(field)) = multipart.try_next().await {
        let content_disposition = field.content_disposition().unwrap();
        let filename = content_disposition.get_filename().unwrap_or("unknown");

        let data = field.bytes().await.map_err(|e| e.to_string())?;

        // In a real application, you would save this to disk or cloud storage
        let path = format!("uploads/{}", filename);
        std::fs::create_dir_all("uploads").map_err(|e| e.to_string())?;

        let mut file = std::fs::File::create(&path).map_err(|e| e.to_string())?;
        file.write_all(&data).map_err(|e| e.to_string())?;

        uploaded_files.push(path);
    }

    Ok(Ok(json!({
        "message": "Files uploaded successfully",
        "files": uploaded_files
    })))
}

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();

    app.post("/upload", upload_handler);

    let _ = app.listen("127.0.0.1:3000").await;
}
```

For more examples and detailed documentation, check out the [Ngyn repository](https://github.com/ngyn-rs/ngyn).
