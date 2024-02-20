# ngyn (`en·jn`)

[![Crates.io](https://img.shields.io/crates/v/ngyn.svg)](https://crates.io/crates/ngyn)
[![Docs.rs](https://docs.rs/ngyn/badge.svg)](https://ngyn.rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)
![MSRV](https://img.shields.io/badge/MSRV-1.63-blue)

A progressive framework in [Rust](https://www.rust-lang.org/) for building scalable web applications through an opinionated architecture.

More information about Ngyn can be found in the [documentation](https://ngyn.rs).

## Features

- Macro API for organizing your application into reusable components
- Built-in dependency injection for managing your application's dependencies
- Asynchronous middleware for handling requests and responses
- Asynchronous routing for defining your application's endpoints
- [Platform-agnostic](#platform-agnosticism) for supporting multiple libraries and frameworks

Please note that Ngyn is still in its early stages of development, and the API is subject to change.

## Example

This example demonstrates how to create a simple web server using Ngyn and [Tide](https://docs.rs/tide). To use Ngyn with Tide, you must enable the `tide` feature in your `Cargo.toml` file.

```toml
[dependencies]
ngyn = { version = "0.3.0", features = ["tide"] }
nject = "0.3.0"
```

And here's the code:

```rust ignore
use ngyn::{
	platforms::{NgynApplication, Result},
	prelude::*
};

#[controller]
struct MyController;

#[routes]
impl MyController {
    #[get("/")]
    fn index(&self) {
        "Hello World!".to_string()
    }

    #[get("/hello/:name")]
    fn hello(&self) {
        let name = request.param("name").unwrap();
        format!("Hello, {}!", name)
    }
}

#[module(controllers = [MyController])]
struct MyAppModule;

#[main]
async fn main() -> Result<()> {
    let app = NgynFactory::<NgynApplication>::create::<MyAppModule>();

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
```

## Philosophy

### Description

Ngyn proposes an opinionated, modular, and scalable architecture, largely inspired by [NestJS](https://nestjs.com/) and structured around the concept of modules - discrete, reusable components that collectively shape an application. These modules, each addressing specific functionalities, can be nested to form a functional hierarchy. This modular design simplifies application organization and enhances reusability across various projects.

### Platform Agnosticism

A platform (more properly called platform engine) in Ngyn refers to the underlying library or framework that is used to build your application. For example, you could build your application using [Actix](https://actix.rs/) or [Warp](https://docs.rs/warp) or [Tide](https://docs.rs/tide), and each of these platforms would provide a different set of features for building your application.

By default, Ngyn uses [Tide](https://docs.rs/tide) as its underlying platform. However, you're not limited to this and can choose to also create your own platform engines.

## Contribution

Ngyn is an open-source project, and we welcome contributions from the community. Feel free to report issues, suggest enhancements, or submit pull requests to help us improve Ngyn.

## License

Ngyn is licensed under the [MIT License](LICENSE.md), allowing you to use, modify, and distribute the framework freely.
