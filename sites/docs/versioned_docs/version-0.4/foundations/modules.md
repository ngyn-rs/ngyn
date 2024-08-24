---
sidebar_position: 4
---

# Modules

Modules are a way to organize your application into reusable components. They are similar to modules in other frameworks and can be used to group related components together.

## Defining Modules

To define a module, you need to implement the `NgynModule` trait for the component. Ideally, you should never have to implement the `NgynModule` trait yourself. Instead, you should use the `#[module]` attribute macro to define your modules. This macro will automatically implement the `NgynModule` trait for you.

```rust
use ngyn::prelude::*;

#[module]
struct AppModule;
```
