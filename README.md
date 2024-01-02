[![Crates.io](https://img.shields.io/crates/v/ngyn.svg)](https://crates.io/crates/ngyn)
[![Docs.rs](https://docs.rs/ngyn/badge.svg)](https://docs.rs/ngyn)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A progressive [platform-agnostic]() framework in [Rust](https://www.rust-lang.org/) for building scalable web applications through an opinionated architecture.

## Features

- Macro-based module system for organizing your application into reusable components
- Built-in dependency injection for managing your application's dependencies
- Asynchronous middleware for handling requests and responses
- Asynchronous routing for defining your application's endpoints
- Platform-agnostic for supporting multiple platforms

Please note that Ngyn is still in its early stages of development, and the API is subject to change.

## Philosophy

Ngyn proposes an opinionated, modular, and scalable architecture, largely inspired by NestJS. It is structured around the concept of modules - discrete, reusable components that collectively shape an application. These modules, each addressing specific functionalities, can be nested to form a functional hierarchy. This modular design simplifies application organization and enhances reusability across various projects.

## Platform Agnosticism

A platform (more properly called platform engine) in Ngyn refers to the underlying library or framework that is used to build your application. For example, you could build your application using [Actix](https://actix.rs/) or [Warp]() or [Tide](), and each of these platforms would provide a different set of features for building your application.

By default, Ngyn uses [Tide](https://docs.rs/tide) as its underlying platform. However, you're not limited to this and can choose to also create your own platform engines.

## Getting Started

### Installation

To get started with Ngyn, simply include the framework in your Rust project by adding the following to your `Cargo.toml`:

```toml
[dependencies]
ngyn = "0.3.0"
```

### Example Usage

Here is a simple example of a Ngyn application without any of the more advanced features.

```rust
use ngyn::{module, NgynFactory, NgynRequest, NgynResponse, Result};

#[module]
struct MyAppModule;

#[ngyn::main]
async fn main() -> Result<()> {
    let app = NgynFactory::create::<MyAppModule>();

    app.get("/", |req: &mut NgynRequest, res: &mut NgynResponse| {
        res.send("Hello World!");
    });

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
```

## Contribution

Ngyn is an open-source project, and we welcome contributions from the community. Feel free to report issues, suggest enhancements, or submit pull requests to help us improve Ngyn.

## License

Ngyn is licensed under the [MIT License](LICENSE), allowing you to use, modify, and distribute the framework freely.

Start building efficient and modularized backend applications with Ngyn today!