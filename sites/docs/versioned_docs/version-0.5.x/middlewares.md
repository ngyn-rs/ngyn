---
sidebar_position: 8
---

# Middleware

Middleware are powerful components that intercept and process HTTP requests and responses before they reach your route handlers or after they leave them. They provide a clean way to separate cross-cutting concerns from your application logic, such as authentication, logging, error handling, and more.

## Understanding Middleware

In Ngyn, middleware follow a simple yet powerful pattern:

1. They receive a mutable request context
2. They can modify the request or perform operations
3. They automatically pass control to the next middleware/handler unless they return early

This pattern allows you to build a processing pipeline that handles various aspects of your application in a modular and reusable way.

## Creating Middleware

To create a middleware, you need to define a struct that implements the `NgynMiddleware` trait. The `NgynMiddleware` trait has an associated async function `handle` that takes a mutable reference to `NgynContext` as argument.

Here's an example of a simple middleware that logs the request path:

```rust
use ngyn::prelude::*;

struct Logger;

impl NgynMiddleware for Logger {
    async fn handle(cx: &mut NgynContext<'_>) {
        println!("Request path: {}", cx.request().uri().path());
        // Middleware processing continues automatically to the next middleware/handler
    }
}
```

In Ngyn v0.5.x, middleware flow control is handled automatically by the framework. You don't need to explicitly call a `next()` method - the request will automatically proceed to the next middleware or handler after your middleware function completes.

## Using Middleware Globally

To apply middleware to your entire application, use the `app.use_middleware` method. Middleware are executed in the order they are added.

```rust
use ngyn::prelude::*;

#[handler]
fn hello() -> &'static str {
    "Hello World!"
}

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();

    app.use_middleware(Logger {});
    app.any("*", hello);

    let _ = app.listen("0.0.0.0:3000").await;
}
```

In this example, the `Logger` middleware will be executed for every request before the `hello` route handler is called.

## Using Middleware for Specific Routes

You can also apply middleware to specific routes by adding them to the route handler using the `middlewares` option in the `#[handler]` attribute.

```rust
#[handler(middlewares = [Logger])]
fn hello() -> &'static str {
    "Hello World!"
}
```

This approach allows you to have specialized middleware for different parts of your application.

## Middleware Execution Order

Middleware are executed in the order they are added to the application or route handler. Global middleware (added with `use_middleware`) are executed before route-specific middleware.

Here's the execution flow:

1. Global middleware (in order of addition)
2. Route-specific middleware (in order specified)
3. Route handler

Example with multiple middleware:

```rust
struct Middleware1;

impl NgynMiddleware for Middleware1 {
    async fn handle(cx: &mut NgynContext<'_>) {
        println!("Middleware 1 - Before");
        // Processing continues to the next middleware automatically
    }
}

struct Middleware2;

impl NgynMiddleware for Middleware2 {
    async fn handle(cx: &mut NgynContext<'_>) {
        println!("Middleware 2 - Before");
        // Processing continues to the next middleware automatically
    }
}

#[handler(middlewares = [Middleware1, Middleware2])]
fn hello() -> &'static str {
    println!("Handler executing");
    "Hello World!"
}
```

Execution output:

```
Middleware 1 - Before
Middleware 2 - Before
Handler executing
```

## Middleware Chaining

Middleware can be chained together to create complex processing pipelines. This allows you to break down complex logic into smaller, reusable components.

```rust
struct AuthMiddleware;

impl NgynMiddleware for AuthMiddleware {
    async fn handle(cx: &mut NgynContext<'_>) {
        // Check for authentication token
        if let Some(token) = cx.request().headers().get("Authorization") {
            // Validate token and proceed
            // Processing continues to the next middleware automatically
        } else {
            // Return unauthorized response
            cx.response_mut().set_status(StatusCode::UNAUTHORIZED);
            cx.response_mut().set_body("Unauthorized".into());
            // In v0.5.x, returning early from the function stops the middleware chain
            return;
        }
    }
}
```

## Common Middleware Use Cases

### Authentication

Verify user identity before allowing access to protected routes:

```rust
struct AuthMiddleware;

impl NgynMiddleware for AuthMiddleware {
    async fn handle(cx: &mut NgynContext<'_>) {
        // Check authentication logic here
        if authenticated {
            // Processing continues to the next middleware automatically
        } else {
            cx.response_mut().set_status(StatusCode::UNAUTHORIZED);
            return; // Stop middleware chain
        }
    }
}
```

### Logging

Log request and response information:

```rust
struct RequestLogger;

impl NgynMiddleware for RequestLogger {
    async fn handle(cx: &mut NgynContext<'_>) {
        let start = std::time::Instant::now();
        let method = cx.request().method().to_string();
        let path = cx.request().uri().path().to_string();

        println!("Request: {} {}", method, path);

        // Note: In v0.5.x, we can't log response status after handler execution
        // You would need to implement a separate response logging mechanism
        // or use a different approach for timing requests
    }
}
```

### CORS (Cross-Origin Resource Sharing)

Handle CORS headers for web applications:

```rust
struct CorsMiddleware;

impl NgynMiddleware for CorsMiddleware {
    async fn handle(cx: &mut NgynContext<'_>) {
        let headers = cx.response_mut().headers_mut();
        headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
        headers.insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE".parse().unwrap());
        headers.insert("Access-Control-Allow-Headers", "Content-Type, Authorization".parse().unwrap());

        // Processing continues to the next middleware automatically
    }
}
```

## Best Practices

1. **Keep middleware focused**: Each middleware should have a single responsibility
2. **Order matters**: Arrange middleware in a logical order (e.g., authentication before authorization)
3. **Error handling**: Implement proper error handling in middleware
4. **Performance**: Be mindful of performance implications, especially for global middleware
5. **Return early to short-circuit**: Return early from the middleware function to stop the middleware chain when needed

For more advanced middleware patterns and examples, check out the [examples](https://github.com/ngyn-rs/ngyn/tree/main/examples) in the Ngyn repository.
