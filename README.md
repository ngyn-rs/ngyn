# ngyn

[![Crates.io](https://img.shields.io/crates/v/ngyn.svg)](https://crates.io/crates/ngyn)
[![Docs.rs](https://docs.rs/ngyn/badge.svg)](https://docs.rs/ngyn)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A progressive [platform-agnostic][] [Rust][] framework for building scalable web applications.

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