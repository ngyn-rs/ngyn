---
sidebar_position: 1
---

# Introduction

Ngyn(/nɪn/) is a modern, async-first web framework for [Rust](https://rust-lang.org) that prioritizes developer ergonomics while maintaining Rust's performance and safety guarantees.
It is designed to be easy to use and easy to learn and built on top of the popular [Hyper](https://hyper.rs/) library.

## Features

- **Intuitive Routing**: Simple, declarative routing patterns `(app.get(), /users/{id})` familiar to web developers
- **Flexible Middleware**: Asynchronous middleware system for request/response processing
- **Performance Focused**: Optimized for both development experience and runtime performance
- **Modern Rust**: Takes advantage of Rust's type system and async features
- **Optional Macros**: Enhance your route handlers with minimal boilerplate
- **Platform Agnostic**: Built to work with various HTTP servers (currently supports Hyper)

> Please note that Ngyn is still in its early stages of development, and the API is subject to change.

## Quick Example

This example demonstrates how to create a simple web server using Ngyn and [Hyper](https://hyper.rs) Platform.

```toml
[dependencies]
ngyn = { version = "0.5" }
tokio = { version = "1", features = ["full"] }
```

And here's the code:

```rust ignore
use ngyn::prelude::*;

#[handler]
fn echo_hello() -> &'static str {
    "Hello World!"
}

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();
    app.any("*", echo_hello); // handle all routes and http methods
    let _ = app.listen("127.0.0.1:8080").await;
}
```

## Contribution

Ngyn is an open-source project, and we welcome contributions from the community. Feel free to report issues, suggest enhancements, or submit pull requests to help us improve Ngyn.

## License

Ngyn is licensed under the [MIT License](https://github.com/ngyn-rs/ngyn/tree/main/LICENSE.md), allowing you to use, modify, and distribute the framework freely.
