---
sidebar_position: 2
---

# Controllers

Controllers are the heart of your application. They are responsible for handling incoming requests, processing them, and returning a response. In Ngyn, controllers are simple Rust structs that implement the `NgynController` trait.

## Defining Controllers

Ideally, you should never have to implement the `NgynController` trait yourself. Instead, you should use the `#[controller]` attribute macro to define your controllers. This macro will automatically implement the `NgynController` trait for you.

```rust
use ngyn::prelude::*;

#[controller]
struct HomeController;
```

The `#[controller]` attribute macro will automatically implement the `NgynController` trait for the `HomeController` struct. This trait provides a async method called `handle` that is responsible for handling incoming requests and a method called `routes` that returns all defined the routes for the controller.

## Defining Routes

Routes are the entry points to your application. They define the URL path, HTTP method, and handler function for each endpoint. In Ngyn, routes can be defined using the `#[route]` attribute macro and should only be used in impl blocks for controllers.

Currently, Ngyn only allows you to define routes for a controller in a single impl block marked with a `#[routes]` macro. This means that you cannot define routes for a controller in multiple impl blocks. This is a limitation of the current implementation and may be changed in the future.

```rust
use ngyn::prelude::*;

#[controller]
struct HomeController;

#[routes]
impl HomeController {
    #[route(GET, "/")]
    async fn index(&self) -> &str {
        "Hello, World!"
    }
}
```

Now, let's break down the code above:

-   The `#[routes]` macro is used to mark the impl block that contains the route definitions for the `HomeController` struct.
-   The `#[route]` macro is used to define a route for the `index` method. The first argument is the HTTP method, and the second argument is the URL path.
-   The `index` method is an async function that returns a reference to a string. This is the handler function for the route.

## Route Parameters

Route parameters are placeholders in the URL path that capture values from the URL. They are defined using arrow braces `<>` in the URL path. In Ngyn, route parameters are automatically extracted from the URL and can be accessed in the handler function using the `Params` transformer.

```rust
use ngyn::prelude::*;

#[controller]
struct UserController;

#[routes]
impl UserController {
    #[route(GET, "/users/<id>")]
    async fn get_user(&self, params: Params) -> String {
        let id = params.get("id").unwrap();
        format!("User ID: {}", id)
    }
}
```

In the example above, we define a route that captures the `id` parameter from the URL path. The `get_user` method takes a `Params` transformer as an argument, which contains all the route parameters extracted from the URL. We can access the `id` parameter using the `get` method on the `Params` transformer. Learn more about `Params` and other transformers in the [Transformers](/docs/providers/transformers) section.

## Route Wildcards

Route wildcards are placeholders in the URL path that match any value. They are defined using an asterisk `*` in the URL path.

```rust
use ngyn::prelude::*;

#[controller]
struct UserController;

#[routes]
impl UserController {
    #[route(GET, "/users/*")]
    async fn get_user(&self) -> &str {
        "All Users"
    }
}
```

## Resource Routes

Resource routes are a convenient way to define a set of routes for a resource (e.g., users, posts, etc.). Ngyn provides convenient `#[get]`, `#[post]`, `#[put]`, `#[patch]`, `#[delete]`, and `#[options]` attribute macros that can be used to define resource routes for a controller. These macros extend the functionality of the `#[route]` macro and automatically generate the URL path based on the method name.

Here's an example of how to define resource routes for a `UserController`:

```rust
use ngyn::prelude::*;

#[controller]
struct UserController;

#[routes]
impl UserController {
    #[get("/users")]
    async fn index(&self) -> &str {
        "List Users"
    }

    #[get("/users/<id>")]
    async fn show(&self, params: Params) -> String {
        let id = params.get("id").unwrap();
        format!("Show User: {}", id)
    }

    #[post("/users")]
    async fn create(&self) -> &str {
        "Create User"
    }

    #[put("/users/<id>")]
    async fn update(&self, params: Params) -> String {
        let id = params.get("id").unwrap();
        format!("Update User: {}", id)
    }

    #[patch("/users/<id>")]
    async fn patch(&self, params: Params) -> String {
        let id = params.get("id").unwrap();
        format!("Patch User: {}", id)
    }

    #[delete("/users/<id>")]
    async fn delete(&self, params: Params) -> String {
        let id = params.get("id").unwrap();
        format!("Delete User: {}", id)
    }

    #[options("/users")]
    async fn options(&self) -> &str {
        "Options"
    }
}
```

## Handler Functions

Handler functions are the functions that are called when a route is matched. They are responsible for processing the incoming request, executing the necessary logic, and returning a response. In Ngyn, handler functions can be async or sync functions that returns any type that implements the `ToBytes` trait.

By default, Ngyn provides a set of implementations for the `ToBytes` trait for common types such as `&str`, `String`, `Vec<u8>`, and `impl ToString`. This means that you can return any of these types from your handler functions without having to worry about converting them to bytes.

However, if you want to return a custom type from your handler functions, you will need to implement the `ToBytes` trait for that type yourself. This trait has a single method called `to_bytes` that takes a reference to the type and returns a `Bytes`.

```rust
use ngyn::prelude::*;

... // HomeController definition

struct MyCustomType {
    ...
}

impl ToBytes for MyCustomType {
    fn to_bytes(&self) -> Bytes {
        // Convert MyCustomType to Bytes
    }
}

#[routes]
impl HomeController {
    #[route(GET, "/")]
    async fn index(&self) -> MyCustomType {
        MyCustomType { ... }
    }
}
```

In the example above, we define a custom type called `MyCustomType` and implement the `ToBytes` trait for it. We then define a route that returns an instance of `MyCustomType` from the `index` method.

**Note:** Ideally, unless you have a good reason to do so, you should never have to implement the `ToBytes` trait yourself. Ngyn provides a [`#[dto]` attribute macro](/docs/advanced/dtos) that can be used to automatically derive the `ToBytes` trait for your custom types.
