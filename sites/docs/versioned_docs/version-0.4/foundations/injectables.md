---
sidebar_position: 3
---

# Injectables

Injectables are a way to share data between different parts of your application. They are similar to services in other frameworks and can be used to store shared state, configuration, or other data that needs to be accessed by multiple components.

Although called "injectables", they are not injected into other components like services in other frameworks. Instead, they are provided to components through initialization strategy and optionally can be further extended by controllers or other components through their `inject` method.

## Defining Injectables

Aside modules, every other component in Ngyn is an injectable. To define an injectable, you need to implement the `NgynInjectable` trait for the component. This trait provides a method called `init` that is responsible for initializing the component and a method called `inject` that is responsible for extending the component.

Ideally, you should never have to implement the `NgynInjectable` trait yourself. Instead, you should use the `#[injectable]` attribute macro to define your injectables. This macro will automatically implement the `NgynInjectable` trait for you.

```rust
use ngyn::prelude::*;

#[injectable]
struct Database {
    connection: String,
}
```

## Initializing Injectables


