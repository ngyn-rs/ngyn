<div align="center">

![ngyn](https://avatars.githubusercontent.com/u/142031159?s=120&v=4)

# ngyn (pronounced "engine")
[![Crates.io](https://img.shields.io/crates/v/ngyn.svg)](https://crates.io/crates/ngyn)
[![Docs.rs](https://docs.rs/ngyn/badge.svg)](https://ngyn.rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)
![MSRV](https://img.shields.io/badge/MSRV-1.81.0-blue)
[![Made in Nigeria](https://img.shields.io/badge/made%20in-nigeria-008751.svg?style=flat-square)](https://github.com/acekyd/made-in-nigeria)

A modern, ergonomic web framework written in Rust for building high-performance web applications.

[Documentation](https://ngyn.rs) | [Getting Started](#getting-started) | [Examples](#examples) | [Contributing](#contributing)

</div>

## Overview

Ngyn is designed to make building web servers in Rust both enjoyable and productive. It combines ergonomic APIs with performance-focused design, making it suitable for everything from small APIs to large-scale web applications.

## Features

- **Intuitive Routing**: Simple, declarative routing patterns `(app.get(), /users/{id})` familiar to web developers
- **Flexible Middleware**: Asynchronous middleware system for request/response processing
- **Performance Focused**: Optimized for both development experience and runtime performance
- **Modern Rust**: Takes advantage of Rust's type system and async features
- **Optional Macros**: Enhance your route handlers with minimal boilerplate
- **Platform Agnostic**: Built to work with various HTTP servers (currently supports Hyper)

## Getting Started

Add ngyn to your `Cargo.toml`:

```toml
[dependencies]
ngyn = "0.5"
tokio = { version = "1", features = ["full"] }
```

Create a basic web server:

```rust ignore
use ngyn::prelude::*;

#[handler]
fn hello() -> &'static str {
    "Hello World!"
}

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();
    
    // Handle all routes and HTTP methods
    app.any("*", hello);
    
    println!("Server running at http://127.0.0.1:8080");
    let _ = app.listen("127.0.0.1:8080").await;
}
```

## Examples

Check out our examples directory for more use cases:
- Basic routing
- Middleware usage
- Authentication
- JSON APIs
- WebSocket handling

## Core Crates

- [`ngyn`][1]: The main framework, reexports all other crates
- [`ngyn_macros`][2]: Procedural macros for route handlers
- [`ngyn_shared`][3]: Core traits and types
- [`ngyn-hyper`][4]: Hyper server integration
- [`ngyn-websocket`][5]: WebSocket support
- [`ngyn-shuttle`][6]: Shuttle.rs deployment service integration
- [`ngyn-vercel`][7]: Vercel deployment service integration

## Roadmap

Ngyn is under active development, with the following features planned for future releases:
- [x] Request and response body handling
- [ ] Form parsing and validation
- [ ] Cookies and sessions management
- [x] Form Handling
- [x] File uploads
- [ ] Response Streaming
- [x] WebSockets
- [x] Async Middlewares
- [x] Async Gates
- [x] Static file serving
- [ ] Internationalization and localization
- [ ] Caching and compression
- [ ] CLI tooling for project generation (In progress)
- [ ] Rate limiting and security features (In progress)
- [ ] Deployment service integrations (In progress)
    - [x] Bare metal (Hyper)
    - [x] Shuttle.rs
    - [x] Vercel
    - [ ] Netlify
    - [ ] Cloudflare Workers
- [ ] Improved documentation and examples (In progress)
    - [x] GraphQL support
    - [ ] Authentication and authorization
- [ ] Error handling and logging
- [ ] Testing utilities

## Performance

Ngyn is designed to be performant while maintaining developer productivity. Some key performance features:
- Zero-cost abstractions
- Efficient routing algorithm
- Minimal allocations
- Async-first design

## How to Contribute

Ngyn thrives on community support and contributions! Here’s how you can get involved:

1. **Report Issues**: Found a bug? Let us know by opening an issue on GitHub.
2. **Suggest Features**: Have an idea for an improvement? Share it with us!
3. **Submit Pull Requests**: Fix bugs or implement new features to help make Ngyn even better.

> If Ngyn has been helpful, consider giving it a star on GitHub to support the project!

Please read our [Contributing Guide](CONTRIBUTING.md) for more details.

## Community

- [GitHub Discussions](https://github.com/ngyn-rs/ngyn/discussions)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/ngyn)

## Status

Ngyn is under active development. While the core API is stabilizing, some features might change. Production use should be carefully evaluated.


## License
Ngyn is licensed under the [MIT License](LICENSE.md). This allows you to use, modify, and distribute the framework freely in your projects.

---

We can’t wait to see what you build with Ngyn! 🚀

[1]:./crates/core
[2]:./crates/macros
[3]:./crates/shared
[4]:./crates/hyper
[5]:./crates/websocket
[6]:./crates/shuttle
[7]:./crates/vercel
