<div align="center">

![ngyn](https://avatars.githubusercontent.com/u/142031159?s=120&v=4)

# ngyn (`en·jn`)

[![Crates.io](https://img.shields.io/crates/v/ngyn.svg)](https://crates.io/crates/ngyn)
[![Docs.rs](https://docs.rs/ngyn/badge.svg)](https://ngyn.rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)
![MSRV](https://img.shields.io/badge/MSRV-1.80-blue)
[![Made in Nigeria](https://img.shields.io/badge/made%20in-nigeria-008751.svg?style=flat-square)](https://github.com/acekyd/made-in-nigeria)

A progressive framework in [Rust](https://www.rust-lang.org/) for building scalable web servers.

More information about Ngyn can be found in the [documentation](https://ngyn.rs).
</div>

## Features

- Battle tested Ergonomic API. (You'll love writing routes in ngyn)
- Performance-balanced approaches
- Optional Macro API for enhancing route handlers
- Asynchronous access gates and middleware for handling requests and responses
- Optional Asynchronous routing support for defining routes

Please note that Ngyn is still in its early stages of development, and the API is subject to change.

## Example

This example demonstrates how to create a simple web server using Ngyn and [Hyper](https://hyper.rs) Platform.

```toml
[dependencies]
ngyn = { version = "0.5" }
```

And here's the code:

```rust ignore
use ngyn::prelude::*;

#[handler]
fn echo_hello() -> String {
    "Hello World!".to_string()
}


#[main]
async fn main() {
    let mut app = HyperApplication::default();
    app.any("*", echo_hello); // handle all routes and http methods
    let _ = app.listen("127.0.0.1:8080").await;
}
```

## Contribution

Ngyn is an open-source project, and we welcome contributions from the community. Feel free to report issues, suggest enhancements, or submit pull requests to help us improve Ngyn.

If this project helped you, looks interesting or you're already making use of it, please drop a star and mention it to others.

## License

Ngyn is licensed under the [MIT License](LICENSE.md), allowing you to use, modify, and distribute the framework freely.
