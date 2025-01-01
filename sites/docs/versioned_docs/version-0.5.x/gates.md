---
sidebar_position: 5
---

# Access Gates

Access gates are a way to control access to your routes based on certain conditions. They are similar to middleware but are used to protect routes from unauthorized access.

## Creating an Access Gate

To create an access gate, you need to define a struct that implements the `NgynGate` trait. The `NgynGate` trait has an associated async function `can_activate` that takes a `NgynContext` as an argument and returns a `bool`.

If an error occurs while checking the condition, you should always return `false` to prevent access to the route, then you can further send a response with the error message.

Here's an example of a simple access gate that checks if the request has a valid API key:

```rust
use ngyn::prelude::*;

struct ApiKeyGate;

impl NgynGate for ApiKeyGate {
    async fn can_activate(ctx: NgynContext) -> bool {
        let api_key = ctx.request().headers().get("x-api-key");

        match api_key {
            Some(key) if key == "secret" => true,
            _ => {
                *ctx.response_mut().status_mut() = StatusCode::FORBIDDEN;
                false
            },
        }
    }
}
```

## Using Access Gates

Access gates are route specific and can be added to the route handler using the `gates` option in the `#[handler]` attribute.

Here's an example of using the `ApiKeyGate` access gate for a specific route:

```rust
#[handler(gates = [ApiKeyGate])]
fn protected_route() -> &'static str {
    "This is a protected route"
}
```

In this example, the `protected_route` handler will only be accessible if the `ApiKeyGate` access gate returns `true`. If the gate returns `false`, the request will be rejected with a `403 Forbidden` response.
