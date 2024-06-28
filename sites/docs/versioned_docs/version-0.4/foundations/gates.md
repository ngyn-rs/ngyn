---
sidebar_position: 5
---

# Gates

Gates are a way to protect your routes from unauthorized access. They are injectables which can be used to perform authentication, authorization, or other checks before allowing a request to proceed.

## Defining Gates

To define a gate, you need to implement the `NgynGate` trait for the component. This trait provides a method called `can_activate` that is responsible for checking if the request should be allowed to proceed.

```rust
use ngyn::prelude::*;

#[injectable]
struct AuthGate;

impl NgynGate for AuthGate {
    fn can_activate(&self, cx: &mut NgynContext, _res: &mut NgynResponse) -> bool {
        // Check if the request has a valid token
        cx.request().headers().get("Authorization").is_some()
    }
}
```

## Using Gates

### Controller-Level Gates

To use a gate, you need to apply it to your routes using the `#[check]` attribute macro.

```rust
use ngyn::prelude::*;

#[controller]
struct MyController;

#[routes]
#[check(AuthGate)]
impl MyController {
    #[get("/")]
    fn index(&self) -> String {
        "Hello World!".to_string()
    }
}
```

### Route-Level Gates

You can also apply a gate to a specific route by using the `#[check]` attribute macro on the route method.

```rust
use ngyn::prelude::*;

#[controller]
struct MyController;

#[routes]
impl MyController {
    #[get("/")]
    #[check(AuthGate)]
    fn index(&self) -> String {
        "Hello World!".to_string()
    }
}
```
