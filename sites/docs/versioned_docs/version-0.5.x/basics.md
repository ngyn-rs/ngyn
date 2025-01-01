---
sidebar_position: 2
---

# Basics

## Creating a new Ngyn Application

To create a new Ngyn application, you need to add the `ngyn` crate to your `Cargo.toml` file:

```toml
[dependencies]
ngyn = "0.5"
tokio = { version = "1", features = ["full"] }
```

Then, you can create a basic web server using the following code:

```rust
#[macro_use] extern crate ngyn;
use ngyn::prelude::*;

#[tokio::main]
async fn main() {
    let mut app = HyperApplication::default();

    // Register routes below

    println!("Server running at http://12");
    let _ = app.listen("0.0.0.0:3000").await;
}
```

## Route Handlers

Route handlers are functions that are called when a route is matched. These functions would need to be marked with `#[handler]` proc macro Here's an example of a simple route handler:

```rust
#[handler]
fn hello() -> &'static str { "Hello World!" }
```

### Complex Route Handlers

For more complex route handlers, these functions take any valid Transduceable Data Structure as an arguments and return a valid response structure. Some supported structures include:

- `&'static str` (string literals)
- `String` (owned strings)
- `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (unsigned integers)
- `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (signed integers)
- `f32`, `f64` (floating point numbers)
- `bool` (boolean values)
- `Option<T>` (optional values)
- `Result<T, E>` (result values, where `T` is a valid response structure and `E` is an error)
- `Vec<T>` (vectors)
- `Vec<u8>` (byte arrays)
- `impl Serialize` and `impl Deserialize` (json based data)
- `JsonResult` (`Result<Value, Value>` dynamic json based data with error handling)

Here's an example of a more complex route handler that returns a JSON response:

```rust
#[handler]
fn hello() -> JsonResult {
    json!({
        "message": "Hello World!"
    })
}
```

### Route URL Parameters

Route handlers can also accept URL parameters as arguments. There are two ways of accessing parameters in route handlers:

1. Using the `Params` struct:
2. Deriving the `Params` trait (recommended)


#### Using the `Params` struct

Here's an example of a route handler that accepts a parameter using the `Params` struct:

```rust
#[handler]
fn hello(params: Params) -> JsonResult {
    let name = params.get("name").unwrap();
    Ok(json!({
        "message": format!("Hello, {}!", name)
    }))
}
```

#### Deriving the `Params` trait

Here's an example of a route handler that accepts a parameter by deriving the `Params` trait:

```rust
#[derive(Params)]
struct HelloParams {
    name: String,
}

#[handler]
fn hello(params: HelloParams) -> JsonResult {
    Ok(json!({
        "message": format!("Hello, {}!", params.name)
    }))
}
```

### Route Query Parameters

Route handlers can also accept query parameters as arguments. There are two ways of accessing query parameters in route handlers:

1. Using the `Query` struct:
2. Deriving the `Query` trait (recommended)

#### Using the `Query` struct

Here's an example of a route handler that accepts query parameters using the `Query` struct:

```rust
#[handler]
fn hello(query: Query) -> JsonResult {
    let name = query.get("name").unwrap();
    Ok(json!({
        "message": format!("Hello, {}!", name)
    }))
}
```

#### Deriving the `Query` trait

Here's an example of a route handler that accepts query parameters by deriving the `Query` trait:

```rust
#[derive(Query)]
struct HelloQuery {
    name: String,
}

#[handler]
fn hello(query: HelloQuery) -> JsonResult {
    Ok(json!({
        "message": format!("Hello, {}!", query.name)
    }))
}
```

## Registering Routes

To register routes in your Ngyn application, you can use the `app` instance created above and call the an HTTP method equivalent method to handle the route.

| Method | Description | Example |
| ------ | ----------- | ------- |
| `app.get` | Register a route that responds to `GET` requests | `app.get("/hello", hello_handler);` |
| `app.post` | Register a route that responds to `POST` requests | `app.post("/hello", hello_handler);` |
| `app.put` | Register a route that responds to `PUT` requests | `app.put("/hello", hello_handler);` |
| `app.delete` | Register a route that responds to `DELETE` requests | `app.delete("/hello", hello_handler);` |
| `app.patch` | Register a route that responds to `PATCH` requests | `app.patch("/hello", hello_handler);` |
| `app.options` | Register a route that responds to `OPTIONS` requests | `app.options("/hello", hello_handler);` |
| `app.head` | Register a route that responds to `HEAD` requests | `app.head("/hello", hello_handler);` |
| `app.connect` | Register a route that responds to `CONNECT` requests | `app.connect("/hello", hello_handler);` |

You can also use the `app.any` method to register a route that responds to all HTTP methods:

```rust
app.any("/hello", hello_handler);
```

## Route Parameters

Route parameters are dynamic parts of a URL that are used to capture values from the URL. These parameters are defined using the `{}` syntax in the route path. Here's an example of a route with a parameter:

```rust
#[handler]
fn user_profile(params: Params) -> JsonResult {
    let user_id = params.get("user_id").unwrap();
    Ok(json!({
        "user_id": user_id
    }))
}

app.get("/users/{user_id}", user_profile);
```

In the example above, the `user_id` parameter is captured from the URL and passed to the `user_profile` route handler.
