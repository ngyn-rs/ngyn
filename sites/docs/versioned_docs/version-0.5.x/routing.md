---
sidebar_position: 3
---

# Routing

Routing is a fundamental concept in web applications. In Ngyn, routing allows you to define how your application responds to client requests to specific endpoints. This guide covers everything you need to know about routing in Ngyn.

## Basic Routing

In Ngyn, you can define routes using HTTP method functions on your application instance:

```rust
use ngyn::prelude::*;

#[handler]
fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();
    
    // Register a route for GET requests to "/hello"
    app.get("/hello", hello);
    
    let _ = app.listen("127.0.0.1:3000").await;
}
```

Ngyn supports all standard HTTP methods:

```rust
// GET request
app.get("/users", get_users);

// POST request
app.post("/users", create_user);

// PUT request
app.put("/users/{id}", update_user);

// DELETE request
app.delete("/users/{id}", delete_user);

// PATCH request
app.patch("/users/{id}", partial_update_user);

// HEAD request
app.head("/status", check_status);

// OPTIONS request
app.options("/users", get_options);
```

## Route Parameters

You can define dynamic route parameters using curly braces `{}` in your route patterns:

```rust
use ngyn::prelude::*;

#[handler]
fn get_user(param: Param) -> String {
    let user_id = param.get("id").unwrap_or("unknown");
    format!("User ID: {}", user_id)
}

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();
    
    // Register a route with a parameter
    app.get("/users/{id}", get_user);
    
    let _ = app.listen("127.0.0.1:3000").await;
}
```

When a request is made to `/users/123`, the `id` parameter will be set to `123`.

## Query Parameters

You can access query parameters using the `Query` struct:

```rust
use ngyn::prelude::*;

#[handler]
fn search(query: Query) -> String {
    let term = query.get("term").unwrap_or("empty");
    format!("Search term: {}", term)
}

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();
    
    app.get("/search", search);
    
    let _ = app.listen("127.0.0.1:3000").await;
}
```

When a request is made to `/search?term=ngyn`, the `term` query parameter will be set to `ngyn`.

## Route Groups

You can group routes with a common prefix using the `group` method:

```rust
use ngyn::prelude::*;

#[handler]
fn list_users() -> &'static str {
    "List of users"
}

#[handler]
fn get_user(param: Param) -> String {
    let user_id = param.get("id").unwrap_or("unknown");
    format!("User ID: {}", user_id)
}

#[handler]
fn create_user() -> &'static str {
    "User created"
}

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();
    
    // Group routes under "/api/users"
    app.group("/api/users", |group| {
        group.get("", list_users);       // Matches "/api/users"
        group.get("/{id}", get_user);   // Matches "/api/users/123"
        group.post("", create_user);     // Matches "/api/users"
    });
    
    let _ = app.listen("127.0.0.1:3000").await;
}
```

## Wildcard Routes

You can use wildcards in your routes to match multiple paths:

```rust
use ngyn::prelude::*;

#[handler]
fn handle_any() -> &'static str {
    "Matched any route"
}

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();
    
    // Match any route
    app.any("*", handle_any);
    
    // Match any route under /api
    app.any("/api/*", handle_any);
    
    let _ = app.listen("127.0.0.1:3000").await;
}
```

## Route Handlers

Route handlers in Ngyn can be defined in several ways:

### Function Handlers

The simplest way is to define a function and mark it with the `#[handler]` attribute:

```rust
#[handler]
fn hello() -> &'static str {
    "Hello, World!"
}
```

### Async Handlers

You can also define async handlers for operations that need to be asynchronous:

```rust
#[handler]
async fn fetch_data() -> String {
    // Simulate an async operation
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    "Data fetched".to_string()
}
```

### Handlers with Context

For more complex scenarios, you can access the request context:

```rust
#[handler]
fn with_context(ctx: NgynContext) -> String {
    let path = ctx.request().uri().path();
    format!("Request path: {}", path)
}
```

## Error Handling

Handlers can return `Result` types to handle errors gracefully:

```rust
#[handler]
fn might_fail() -> Result<String, String> {
    // Simulate a condition that might fail
    if rand::random::<bool>() {
        Ok("Success!".to_string())
    } else {
        Err("Something went wrong".to_string())
    }
}
```

For more advanced routing scenarios, check out the [examples](https://github.com/ngyn-rs/ngyn/tree/main/examples) in the Ngyn repository.