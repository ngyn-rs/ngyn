---
sidebar_position: 1
---

# Introduction

Ngyn(/nɪn/) is a simple, fast, and lightweight web framework for building web applications in [Rust](https://rust-lang.org). It is designed to be easy to use and easy to learn. Ngyn is built on top of the popular [Hyper](https://hyper.rs/) library and provides a simple and easy-to-use API for building web applications.

## Features

-   Macro API for organizing your application into reusable components
-   Asynchronous middleware for handling requests and responses
-   Asynchronous routing support for defining routes
-   [Platform-agnostic](#platform-agnosticism) for supporting multiple libraries and frameworks

Please note that Ngyn is still in its early stages of development, and the API is subject to change.

## Example

This example demonstrates how to create a simple web server using Ngyn and [Hyper](https://hyper.rs) Platform.

```toml
[dependencies]
ngyn = { version = "0.4" }
```

And here's the code:

```rust ignore
use ngyn::prelude::*;

#[controller]
struct MyController;

#[routes]
impl MyController {
    #[get("/")]
    fn index(&self) -> String {
        "Hello World!".to_string()
    }

    #[get("/hello/<name>")]
    fn hello(&self, param: Param) -> String {
        let name = param.get("name").unwrap();
        format!("Hello, {}!", name)
    }
}

#[module(controllers = [MyController])]
struct MyAppModule;

#[main]
async fn main() {
    let app = NgynFactory::<HyperApplication>::create::<MyAppModule>();
    let _ = app.listen("127.0.0.1:8080").await;
}
```

## Philosophy

### Description

Ngyn proposes an opinionated, modular, and scalable architecture, largely inspired by [NestJS](https://nestjs.com/) and structured around the concept of modules - discrete, reusable components that collectively shape an application. These modules, each addressing specific functionalities, can be nested to form a functional hierarchy. This modular design simplifies organization and enhances reusability across various projects.

### Core Principles

-   **Modularity**: Ngyn is designed to be modular, with each module addressing a specific functionality. This modular design simplifies organization and enhances reusability across various projects.
-   **Scalability**: Ngyn is designed to be scalable, with the ability to handle large amounts of traffic and requests. It is built on top of the Hyper library, which is known for its high performance and scalability.
-   **Simplicity**: Ngyn is designed to be simple and easy to use. It provides a high-level API for building web applications, making it easy to write web servers in Rust.
-   **Flexibility**: Ngyn is designed to be flexible, allowing you to choose the platform that best suits your needs. By default, Ngyn uses Hyper as its underlying platform, but you can also create your own platform engines.
-   **Performance**: Ngyn is designed to be fast and efficient, with low latency and high throughput. It is built on top of the Hyper library, which is known for its high performance and efficiency.

### Platform Agnosticism

A platform (more properly called platform engine) in Ngyn refers to the underlying library or framework that is used to build your application. For example, you could build your web server using [Actix](https://actix.rs/) or [Warp](https://docs.rs/warp) or [Tide](https://docs.rs/tide), and each of these platforms would provide a different set of features for building your web server.

By default, Ngyn uses [Hyper](https://hyper.rs) as its underlying platform. However, you're not limited to this and can choose to also create your own platform engines.

## Contribution

Ngyn is an open-source project, and we welcome contributions from the community. Feel free to report issues, suggest enhancements, or submit pull requests to help us improve Ngyn.

## License

Ngyn is licensed under the [MIT License](LICENSE.md), allowing you to use, modify, and distribute the framework freely.
