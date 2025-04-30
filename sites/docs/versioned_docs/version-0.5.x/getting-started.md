---
sidebar_position: 2
---

# Getting Started

This guide will help you get started with Ngyn, a modern, async-first web framework for Rust. We'll walk through setting up a new project, creating routes, and running your first Ngyn application.

## Prerequisites

Before you begin, make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)

## Creating a New Project

Let's create a new Rust project:

```bash
cargo new my_ngyn_app
cd my_ngyn_app
```

## Adding Ngyn Dependencies

Add Ngyn to your `Cargo.toml` file:

```toml
[dependencies]
ngyn = "0.5"
tokio = { version = "1", features = ["full"] }
```

## Creating Your First Application

Replace the contents of `src/main.rs` with the following code:

```rust
use ngyn::prelude::*;

#[handler]
fn hello() -> &'static str {
    "Hello, Ngyn!"
}

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();
    
    // Register a route
    app.get("/", hello);
    
    println!("Server running at http://127.0.0.1:3000");
    let _ = app.listen("127.0.0.1:3000").await;
}
```

## Running Your Application

Run your application with:

```bash
cargo run
```

You should see the message "Server running at http://127.0.0.1:3000". Open your browser and navigate to [http://127.0.0.1:3000](http://127.0.0.1:3000) to see "Hello, Ngyn!".

## Understanding the Code

Let's break down what's happening in our application:

1. **Imports**: We import everything we need from the Ngyn prelude.

2. **Route Handler**: We define a function `hello` that returns a static string. The `#[handler]` attribute marks this function as a route handler.

3. **Application Setup**: We create a new Hyper-based application with `HyperApplication::default()`.

4. **Route Registration**: We register the `hello` function to handle GET requests to the root path (`/`).

5. **Server Start**: We start the server on localhost port 3000.

## Next Steps

Now that you have a basic Ngyn application running, you can explore more features:

- [Routing](./routing.md) - Learn about different routing patterns
- [Middleware](./middlewares.md) - Add middleware to your application
- [Forms Handling](./forms.md) - Process form submissions
- [Access Gates](./gates.md) - Control access to your routes

Or check out the [examples](https://github.com/ngyn-rs/ngyn/tree/main/examples) in the Ngyn repository for more complex applications.