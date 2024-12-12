---
sidebar_position: 1
---

# Introduction

Ngyn(/nɪn/) is a simple, fast, and lightweight web framework for building web applications in [Rust](https://rust-lang.org). It is designed to be easy to use and easy to learn. Ngyn is built on top of the popular [Hyper](https://hyper.rs/) library and provides a simple and easy-to-use API for building web applications.

## Features

- Battle tested Ergonomic API. (You'll love writing routes in ngyn)
- Performance-balanced approaches
- Optional Macro API for enhancing route handlers
- Asynchronous access gates and middleware for handling requests and responses
- Optional Asynchronous routing support for defining routes

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
fn echo_hello() -> String {
    "Hello World!".to_string()
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

Ngyn is licensed under the [MIT License](LICENSE.md), allowing you to use, modify, and distribute the framework freely.
