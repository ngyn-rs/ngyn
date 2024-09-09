---
sidebar_position: 6
---

# Middlewares

Middlewares are a way to intercept incoming requests and outgoing responses. They are injectables that can be used to perform operations such as logging, error handling, or modifying the request/response before it reaches the controller.

## Defining Middlewares

To define a middleware, you need to implement the `NgynMiddleware` trait for the component. This trait provides a method called `handle` that is responsible for processing the request and response.

```rust title="src/middlewares/logger.rs"
use ngyn::prelude::*;

#[injectable]
struct Logger;

impl NgynMiddleware for Logger {
    fn handle(&self, cx: &mut NgynContext, res: &mut NgynResponse) {
        // Log the request method and path
        log::info!("{} {}", cx.request().method(), cx.request().uri().path());
    }
}
```

## Using Middlewares

### Controller-Level Middlewares

To use a middleware, you need to apply it to your routes through the `#[controller]` attribute macro.

```rust
use ngyn::prelude::*;

#[controller(middlewares = [Logger])]
struct MyController;
```

### Global Middlewares

You can also apply a middleware globally to all routes by using it through your application factory.

```rust title="src/main.rs"
use ngyn::prelude::*;

#[main]
async fn main() {
    let mut app = NgynFactory::<HyperApplication>::create::<AppModule>();

    app.use_middleware(Logger::new());

    let _ = app.listen("0.0.0.0:3000").await;
}
```

### Route-Level Middlewares (Coming Soon)

You can also apply a middleware to a specific route by using the `#[middleware]` attribute macro on the route method.

```rust
use ngyn::prelude::*;

#[controller]
struct MyController;

#[routes]
impl MyController {
    #[get("/")]
    #[middleware(Logger)]
    fn index(&self) -> String {
        "Hello World!".to_string()
    }
}
```

## Middleware Execution Order

Middlewares are executed in the order they are defined. The order of execution is important because it determines the sequence in which the middlewares are applied to the request and response.

```rust
use ngyn::prelude::*;

#[injectable]
struct Middleware1;

impl NgynMiddleware for Middleware1 {
    fn handle(&self, cx: &mut NgynContext, res: &mut NgynResponse) {
        log::info!("Middleware1 - Before");
    }
}

#[injectable]
struct Middleware2;

impl NgynMiddleware for Middleware2 {
    fn handle(&self, cx: &mut NgynContext, res: &mut NgynResponse) {
        log::info!("Middleware2 - Before");
    }
}

#[controller(middlewares = [Middleware1, Middleware2])]
struct MyController;
```
