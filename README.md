# Ngyn

> A progressive Rust Framework for building efficient and modularized backend applications

Ngyn is a powerful and progressive Rust framework crafted for the development of efficient and modularized backend applications. With a focus on performance, reliability, and flexibility, Ngyn empowers developers to build robust server-side solutions with ease. Ngyn comes packed with powerful macros, utilities, and features that make it easy to build performant and secure applications.

## Features

- **Progressive Enhancement:** Ngyn embraces the philosophy of progressive enhancement, allowing developers to start with a foundational set of features and progressively enhance the application as needed. This ensures a smooth and adaptable development process.

- **Efficiency at Core:** Ngyn is designed to prioritize efficiency in resource utilization, providing a performant environment for backend applications. Whether handling data processing, managing business logic, or interfacing with databases, Ngyn ensures optimal performance.

- **Modular Architecture:** Ngyn encourages a modularized approach to application development. Break down your backend logic into independent and reusable modules, promoting code organization, maintainability, and scalability. Ngyn's modular architecture facilitates collaboration among developers working on distinct components.

- **Optional Async:** Ngyn provides support for async operations out of the box through controllers. However, it's 100% optional.

- **Powerful Macros:** Ngyn provides a set of powerful macros that simplify common tasks. From defining routes to creating middleware, Ngyn's macros make it easy to build robust applications.

- **Lightweight:** Ngyn is lightweight and leaves a minimal footprint, making it a great choice for projects of all sizes. Ngyn's lightweight nature ensures that your application is not bogged down by unnecessary bloat, yet still provides the features you need to build a robust backend.

- **Fully Extensible:** Ngyn allows you to build your own platform engines or make use of any of the built-in `vercel` or `tide` platform engines.

## Getting Started

### Installation

To get started with Ngyn, simply include the framework in your Rust project by adding the following to your `Cargo.toml`:

```toml
[dependencies]
ngyn = "0.2.8"
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

    app.get("/", |req: &NgynRequest, res: &mut NgynResponse| {
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