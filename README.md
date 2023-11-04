# Ngyn

Ngyn is a lightweight and powerful framework for creating robust applications in Rust. It provides a modern set of macros, and utilities that makes it easy to build performant and secure applications.


## Features

- **Modular Structure**: Ngyn uses a modular structure that makes it easy to organize your code and keep it maintainable.
- **Powerful Macros**: Ngyn provides a set of powerful macros that simplify common tasks.
- **Lightweight**: Ngyn is lightweight and has a minimal footprint, making it a great choice for projects of all sizes.
- **High Performance**: Ngyn is built with performance in mind. It leverages the power of Rust to deliver fast, efficient web apps.
- **Route Gates** (WIP): A simplified way to restrict access to a route based on a set condition

## Get Started

Ngyn is easy to get started with. All you need is a basic understanding of Rust and the basics of the Ngyn API. There is a comprehensive documentation available that covers all of the features of Ngyn, as well as sample code to help you get started quickly. 

Ngyn is a great choice for creating robust applications quickly and easily. With its flexible and powerful API, powerful macros, and robust security features, you can be sure that your applications are secure and performant. 

```rust
use modules::MyAppModule;
use ngyn::{NgynFactory, NgynRequest, NgynResponse, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let app = NgynFactory::create::<MyAppModule>();

    app.get("/", |req: &NgynRequest, res: &mut NgynResponse| {
        res.send("Hello World!");
    });

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
```

```rust
// my_gate.rs
// Define a gate to restrict access to a resource
use ngyn::NgynGate;

pub struct MyGate;

impl NgynGate for MyGate {
    fn can_activate(&self, req: &NgynRequest) -> bool {
        // Check if the user is authenticated
        if req.user.is_authenticated() {
            return true;
        }
        false
    }
}
```
