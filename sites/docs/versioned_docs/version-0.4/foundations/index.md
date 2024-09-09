---
sidebar_position: 2
---

# Foundations

## Overview

Ngyn is a web framework for Rust that is designed to be fast, secure, and easy to use. It is built on top of the [Hyper](https://hyper.rs) library, which is a fast and low-level HTTP library for Rust. Ngyn provides a high-level API for building web applications, making it easy to write web servers in Rust.

## Pre-requisites

Before you start using Ngyn, you should have a basic understanding of Rust and web development. You should also have Rust installed. If you don't have Rust installed, you can follow the instructions [here](https://www.rust-lang.org/tools/install).

By practice, Ngyn MSRV (Minimum Supported Rust Version) is set to `1.75.0`. This means that you need to have Rust version `1.75.0` or higher installed on your system to use Ngyn.

## Setting up a new project

To create a new Ngyn project, you can use `cargo-generate`. For example, to create a new project called `my_project`, you can run the following command:

```bash
cargo generate --git https://github.com/ngyn-rs/ngyn-starter.git --name my_project
```

This will create a new directory called `my_project` with a basic Ngyn project structure. You can then navigate to the `my_project` directory and run `cargo run` to start the server.

### Basic Apps

Here is a simple example of a Ngyn web server powered by hyper that responds with `"Hello, World!"` to all `GET` requests:

```rust
use ngyn::prelude::*;

#[ngyn::main]
async fn main() {
    let mut app = HyperApplication::default();

    app.get("*", |cx: &mut NgynContext, res: &mut NgynResponse| {
        res.send("Hello, World!");
    });

    let _ = app.listen("0.0.0.0:3000").await;
}
```

This example code above uses platform methods to add a route that responds with `"Hello, World!"` to all `GET` requests. The `listen` method starts the server and listens on port `3000`. Other platform methods for adding routes include `post`, `put`, `delete`, and `patch`. You can learn more about the available methods in the [Platforms](/docs/platforms) section.

### Advanced Apps

Ngyn also supports more advanced features like middleware, error handling, and request/response manipulation through controllers. Here is an example of a Ngyn web server that is initialized with a custom module:

```rust
use ngyn::prelude::*;

#[module]
struct AppModule;

#[ngyn::main]
async fn main() {
    let mut app = NgynFactory::<HyperApplication>::create::<AppModule>();

    let _ = app.listen("0.0.0.0:3000").await;
}
```

The next step is to create a controller that will handle incoming requests. You would learn this in the next section.
