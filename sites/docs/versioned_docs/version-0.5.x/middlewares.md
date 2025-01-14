---
sidebar_position: 3
---

# Middleware

Middleware are used to modify the request and response objects before they are processed by the route handler. Middleware can be used to perform tasks such as logging, authentication, rate limiting, and more.

## Creating Middleware

To create a middleware, you need to define a struct that implements the `NgynMiddleware` trait. The `NgynMiddleware` trait has an associated async function `handle` that takes a `NgynContext` as argument.

Here's an example of a simple middleware that logs the request path:

```rust
use ngyn::prelude::*;

struct Logger;

impl NgynMiddleware for Logger {
    async fn handle(ctx: NgynContext) {
        println!("Request path: {}", ctx.request().uri().path());
    }
}
```

## Using Middleware Globally

To use middleware in your application, you can add them to the application using the `app.use_middleware` method. Middleware are executed in the order they are added.

Here's an example of using the `Logger` middleware in an application:

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

## Using Middleware Locally

You can also use middleware for specific routes by adding them to the route handler using the `middlewares` option in the `#[handler]` attribute.

Here's an example of using the `Logger` middleware for a specific route:

```rust
#[handler(middlewares = [Logger])]
fn hello() -> &'static str {
    "Hello World!"
}
```

In this example, the `Logger` middleware will only be executed for the `hello` route handler.

## Middleware Execution Order

Middleware are executed in the order they are added to the application or route handler. Middleware added globally will be executed before middleware added locally to a route handler.

Here's an example of using multiple middleware in an application:

```rust

struct Middleware1;

impl NgynMiddleware for Middleware1 {
    async fn handle(ctx: NgynContext) {
        println!("Middleware 1");
    }
}

struct Middleware2;

impl NgynMiddleware for Middleware2 {
    async fn handle(ctx: NgynContext) {
        println!("Middleware 2");
    }
}

#[handler(middlewares = [Middleware1, Middleware2])] // Middleware 1 and Middleware 2
fn hello() -> &'static str {
    "Hello World!"
}
```

In this example, `Middleware1` will be executed before `Middleware2` for the `hello` route handler.

## Middleware Chaining

Middleware can be chained together to create complex processing pipelines. Middleware chaining allows you to break down complex processing logic into smaller, reusable components.

Here's an example of chaining multiple middleware together:

```rust
struct Middleware1;

impl NgynMiddleware for Middleware1 {
    async fn handle(ctx: NgynContext) {
        println!("Middleware 1");
    }
}

struct Middleware2;

impl NgynMiddleware for Middleware2 {
    async fn handle(ctx: NgynContext) {
        Middleware1::handle(ctx).await;
        println!("Middleware 2");
    }
}

#[handler(middlewares = [Middleware2])] // Middleware 1 and Middleware 2
fn hello() -> &'static str {
    "Hello World!"
}
```

In this example, `Middleware1` will be executed before `Middleware2` for the `hello` route handler.
