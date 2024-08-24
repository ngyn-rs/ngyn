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

Injectables are initialized when the application is created. You can provide the initialization strategy for an injectable by supplying a value to `init` argument of the `#[injectable]` attribute macro. The value must be a string literal that corresponds to the name of the initialization method.

```rust
use ngyn::prelude::*;

#[injectable(init = "init_db")]
struct Database {
    connection: String,
}

impl Database {
    fn init_db() -> Self {
        Self {
            connection: "mongodb://localhost:27017".to_string(),
        }
    }
}
```

## Using Injectables

Injectables can be accessed in controllers, middlewares, gates, and other components by declaring them as part of the component's fields. The injectable will be automatically initialized and provided to the component when it is created.

```rust
use ngyn::prelude::*;

#[controller]
struct MyController {
    db: Database,
}

impl MyController {
    fn index(&self) -> String {
        format!("Database connection: {}", self.db.connection)
    }
}
```

## Extending Injectables

Injectables can be further extended by controllers or other components through their `inject` method. This method is called after the injectable is initialized and can be used to modify the injectable's state or provide additional functionality.

```rust
use ngyn::prelude::*;

#[injectable(init = "init_db", inject = "custom_inject")]
struct Database {
    connection: String,
}

impl Database {
    fn init_db() -> Self {
        Self {
            connection: "mongodb://localhost:27017".to_string(),
        }
    }

    fn custom_inject(&mut self) {
        self.connection = "mongodb://localhost:27017".to_string();
    }
}
```
