---
sidebar_position: 4
---

# Request and Response Handling

Understanding how to work with HTTP requests and responses is essential for building web applications. Ngyn provides a clean and intuitive API for handling requests and generating responses.

## The Request Object

In Ngyn, you can access the request object through the `NgynContext` or directly as a parameter in your handler function:

```rust
use ngyn::prelude::*;

#[handler]
fn handle_request(req: NgynRequest) -> String {
    let method = req.method();
    let uri = req.uri();
    
    format!("Received a {} request to {}", method, uri)
}
```

### Accessing Headers

You can access request headers using the `headers()` method:

```rust
#[handler]
fn check_auth(req: NgynRequest) -> Result<String, String> {
    let auth_header = req.headers().get("Authorization");
    
    match auth_header {
        Some(auth) => Ok(format!("Authorized with: {}", auth.to_str().unwrap_or("invalid header"))),
        None => Err("Unauthorized".to_string()),
    }
}
```

### Reading the Request Body

To read the request body, you can use the `Body` struct:

```rust
#[handler]
async fn read_body(body: Body) -> Result<String, String> {
    match body.text().await {
        Ok(text) => Ok(format!("Received body: {}", text)),
        Err(_) => Err("Failed to read body".to_string()),
    }
}
```

### Parsing JSON

For JSON requests, you can parse the body into a structured format:

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    email: String,
}

#[handler]
async fn create_user(body: Body) -> JsonResult {
    match body.json::<User>().await {
        Ok(user) => Ok(json!({
            "status": "success",
            "user": user
        })),
        Err(_) => Ok(json!({
            "status": "error",
            "message": "Invalid JSON"
        })),
    }
}
```

## The Response Object

In Ngyn, you can return various types from your handler functions, which will be automatically converted to HTTP responses:

### Simple Responses

```rust
#[handler]
fn hello() -> &'static str {
    "Hello, World!"
}

#[handler]
fn json_response() -> JsonResult {
    Ok(json!({
        "message": "This is a JSON response"
    }))
}
```

### Custom Status Codes

You can set custom status codes by accessing the response object through the context:

```rust
#[handler]
fn not_found(ctx: NgynContext) -> &'static str {
    *ctx.response_mut().status_mut() = http::StatusCode::NOT_FOUND;
    "Resource not found"
}
```

### Setting Headers

You can set response headers in a similar way:

```rust
#[handler]
fn with_headers(ctx: NgynContext) -> &'static str {
    let headers = ctx.response_mut().headers_mut();
    headers.insert("X-Custom-Header", "custom-value".parse().unwrap());
    headers.insert("Content-Type", "text/plain".parse().unwrap());
    
    "Response with custom headers"
}
```

### Streaming Responses

For large responses, you might want to stream the data instead of loading it all into memory:

```rust
use tokio::fs::File;
use tokio_util::io::ReaderStream;

#[handler]
async fn stream_file(ctx: NgynContext) -> Result<NgynResponse, String> {
    let file = match File::open("large_file.txt").await {
        Ok(file) => file,
        Err(_) => return Err("File not found".to_string()),
    };
    
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    
    let mut response = NgynResponse::new(body);
    *response.status_mut() = http::StatusCode::OK;
    response.headers_mut().insert(
        "Content-Type", 
        "text/plain".parse().unwrap()
    );
    
    Ok(response)
}
```

## Working with Cookies

Ngyn provides utilities for working with cookies:

```rust
#[handler]
fn set_cookie(ctx: NgynContext) -> &'static str {
    let headers = ctx.response_mut().headers_mut();
    headers.insert(
        "Set-Cookie", 
        "session=123; HttpOnly; Path=/".parse().unwrap()
    );
    
    "Cookie set"
}

#[handler]
fn read_cookie(req: NgynRequest) -> String {
    let cookies = req.headers().get("Cookie");
    
    match cookies {
        Some(cookie_header) => {
            let cookie_str = cookie_header.to_str().unwrap_or("");
            format!("Cookies: {}", cookie_str)
        },
        None => "No cookies found".to_string(),
    }
}
```

## Redirects

You can perform redirects by setting the appropriate status code and location header:

```rust
#[handler]
fn redirect(ctx: NgynContext) -> &'static str {
    let mut response = ctx.response_mut();
    *response.status_mut() = http::StatusCode::FOUND; // 302 redirect
    response.headers_mut().insert(
        "Location", 
        "/new-location".parse().unwrap()
    );
    
    "Redirecting..."
}
```

## Error Handling

Ngyn allows you to return `Result` types from your handlers to handle errors gracefully:

```rust
#[handler]
fn might_fail() -> Result<String, String> {
    // Simulate a condition that might fail
    if rand::random::<bool>() {
        Ok("Success!".to_string())
    } else {
        Err("Something went wrong".to_string())
    }
}
```

When an `Err` is returned, Ngyn will automatically convert it to a 500 Internal Server Error response with the error message as the body.

For more advanced request and response handling, check out the [examples](https://github.com/ngyn-rs/ngyn/tree/main/examples) in the Ngyn repository.