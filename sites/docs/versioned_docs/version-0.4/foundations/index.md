---
sidebar_position: 1
---

# Overview

Ngyn is a web framework for Rust that is designed to be fast, secure, and easy to use. It is built on top of the [Hyper](https://hyper.rs) library, which is a fast and low-level HTTP library for Rust. Ngyn provides a high-level API for building web applications, making it easy to write web servers in Rust.

## Pre-requisites

Before you start using Ngyn, you should have a basic understanding of Rust and web development. You should also have Rust installed. If you don't have Rust installed, you can follow the instructions [here](https://www.rust-lang.org/tools/install).

By practice, Ngyn MSRV (Minimum Supported Rust Version) is set to `1.63.0` (always the same as hyper). This means that you need to have Rust version `1.63.0` or higher installed on your system to use Ngyn.

## Setting up a new project

To create a new Ngyn project, you can use `cargo-generate`. For example, to create a new project called `my_project`, you can run the following command:

```bash
cargo generate --git https://github.com/ngyn-rs/ngyn-starter.git --name my_project
```

This will create a new directory called `my_project` with a basic Ngyn project structure. You can then navigate to the `my_project` directory and run `cargo run` to start the server.

## Hello, World!

Here is a simple example of a Ngyn web server powered by hyper that responds with `"Hello, World!"` to all `GET` requests:

```rust
use ngyn::prelude::*;

#[ngyn::main]
async fn main() {
    let mut app = HyperApplication::default();

    app.get("*", |req, res| {
        res.send("Hello, World!");
    });

    let _ = app.listen("0.0.0.0:3000").await;
}
```
