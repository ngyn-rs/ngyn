---
sidebar_position: 8
---

# Testing

Testing is a crucial part of developing robust web applications. Ngyn provides several ways to test your application, from unit tests to integration tests. This guide will show you how to effectively test your Ngyn applications.

## Unit Testing

Unit tests focus on testing individual components of your application in isolation. In Ngyn, you can unit test your route handlers, middleware, and access gates.

### Testing Route Handlers

Here's an example of how to unit test a route handler:

```rust
use ngyn::prelude::*;

#[handler]
fn hello() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hello_handler() {
        // Call the handler directly
        let result = hello();
        assert_eq!(result, "Hello, World!");
    }
}
```

### Testing Handlers with Parameters

For handlers that take parameters, you can create mock objects:

```rust
use ngyn::prelude::*;

#[handler]
fn greet(param: Param) -> String {
    let name = param.get("name").unwrap_or("Guest");
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_greet_handler() {
        // Create a mock Param object
        let mut params = HashMap::new();
        params.insert("name".to_string(), "John".to_string());
        let param = Param::from(params);
        
        // Call the handler with the mock param
        let result = greet(param);
        assert_eq!(result, "Hello, John!");
    }
}
```

## Integration Testing

Integration tests verify that different parts of your application work together correctly. For Ngyn applications, this often means testing the HTTP endpoints.

### Setting Up Integration Tests

Create a test module in your project:

```rust
// tests/integration_test.rs
use ngyn::prelude::*;
use reqwest;

// Import your application code
use your_app::create_app;

#[tokio::test]
async fn test_hello_endpoint() {
    // Start your application in the background
    let app = create_app();
    let server_handle = tokio::spawn(async move {
        let _ = app.listen("127.0.0.1:3001").await;
    });
    
    // Give the server a moment to start
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    
    // Make a request to your application
    let client = reqwest::Client::new();
    let response = client.get("http://127.0.0.1:3001/hello")
        .send()
        .await
        .expect("Failed to send request");
    
    // Assert on the response
    assert_eq!(response.status(), 200);
    let body = response.text().await.expect("Failed to read response body");
    assert_eq!(body, "Hello, World!");
    
    // Shutdown the server
    server_handle.abort();
}
```

### Using a Test Client

For more complex integration tests, you might want to create a test client:

```rust
// tests/test_client.rs
use ngyn::prelude::*;
use reqwest::{Client, Response};

pub struct TestClient {
    client: Client,
    base_url: String,
}

impl TestClient {
    pub fn new(port: u16) -> Self {
        Self {
            client: Client::new(),
            base_url: format!("http://127.0.0.1:{}", port),
        }
    }
    
    pub async fn get(&self, path: &str) -> Response {
        self.client.get(format!("{}{}", self.base_url, path))
            .send()
            .await
            .expect("Failed to send GET request")
    }
    
    pub async fn post(&self, path: &str, json: serde_json::Value) -> Response {
        self.client.post(format!("{}{}", self.base_url, path))
            .json(&json)
            .send()
            .await
            .expect("Failed to send POST request")
    }
    
    // Add more methods for other HTTP methods as needed
}
```

Then use it in your tests:

```rust
#[tokio::test]
async fn test_api() {
    // Start your application
    let app = create_app();
    let port = 3002;
    let server_handle = tokio::spawn(async move {
        let _ = app.listen(format!("127.0.0.1:{}", port)).await;
    });
    
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    
    // Create a test client
    let client = TestClient::new(port);
    
    // Test GET endpoint
    let response = client.get("/users").await;
    assert_eq!(response.status(), 200);
    
    // Test POST endpoint
    let create_user = serde_json::json!({
        "name": "John Doe",
        "email": "john@example.com"
    });
    let response = client.post("/users", create_user).await;
    assert_eq!(response.status(), 201);
    
    // Shutdown the server
    server_handle.abort();
}
```

## Testing Middleware

To test middleware, you can create a mock context and call the middleware's `handle` method:

```rust
use ngyn::prelude::*;

struct AuthMiddleware;

impl NgynMiddleware for AuthMiddleware {
    async fn handle(ctx: NgynContext) {
        let auth_header = ctx.request().headers().get("Authorization");
        if auth_header.is_none() {
            *ctx.response_mut().status_mut() = http::StatusCode::UNAUTHORIZED;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::{Request, Response};
    
    #[tokio::test]
    async fn test_auth_middleware() {
        // Create a mock request
        let request = Request::builder()
            .uri("/protected")
            .body(Body::empty())
            .unwrap();
        
        // Create a mock response
        let response = Response::builder()
            .status(200)
            .body(Body::empty())
            .unwrap();
        
        // Create a mock context
        let ctx = NgynContext::new(request, response);
        
        // Call the middleware
        AuthMiddleware::handle(ctx.clone()).await;
        
        // Assert on the response status
        assert_eq!(ctx.response().status(), http::StatusCode::UNAUTHORIZED);
    }
}
```

## Testing Access Gates

Similarly, you can test access gates by creating a mock context:

```rust
use ngyn::prelude::*;

struct ApiKeyGate;

impl NgynGate for ApiKeyGate {
    async fn can_activate(ctx: NgynContext) -> bool {
        let api_key = ctx.request().headers().get("x-api-key");
        match api_key {
            Some(key) if key == "secret" => true,
            _ => {
                *ctx.response_mut().status_mut() = http::StatusCode::FORBIDDEN;
                false
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::{Request, Response, header::HeaderValue};
    
    #[tokio::test]
    async fn test_api_key_gate_with_valid_key() {
        // Create a mock request with a valid API key
        let request = Request::builder()
            .uri("/protected")
            .header("x-api-key", "secret")
            .body(Body::empty())
            .unwrap();
        
        // Create a mock response
        let response = Response::builder()
            .status(200)
            .body(Body::empty())
            .unwrap();
        
        // Create a mock context
        let ctx = NgynContext::new(request, response);
        
        // Call the gate
        let result = ApiKeyGate::can_activate(ctx.clone()).await;
        
        // Assert that the gate allows access
        assert!(result);
    }
    
    #[tokio::test]
    async fn test_api_key_gate_with_invalid_key() {
        // Create a mock request with an invalid API key
        let request = Request::builder()
            .uri("/protected")
            .header("x-api-key", "wrong-key")
            .body(Body::empty())
            .unwrap();
        
        // Create a mock response
        let response = Response::builder()
            .status(200)
            .body(Body::empty())
            .unwrap();
        
        // Create a mock context
        let ctx = NgynContext::new(request, response);
        
        // Call the gate
        let result = ApiKeyGate::can_activate(ctx.clone()).await;
        
        // Assert that the gate denies access
        assert!(!result);
        assert_eq!(ctx.response().status(), http::StatusCode::FORBIDDEN);
    }
}
```

## Mocking External Dependencies

For tests that involve external dependencies like databases or APIs, you can use mocking libraries like `mockall` to create mock implementations:

```rust
use mockall::predicate::*;
use mockall::*;

#[automock]
trait UserRepository {
    async fn find_user(&self, id: i32) -> Option<User>;
    async fn create_user(&self, user: User) -> Result<User, String>;
}

#[handler]
async fn get_user(param: Param, repo: UserRepositoryMock) -> Result<User, String> {
    let user_id = param.get("id").unwrap_or("0").parse::<i32>().unwrap_or(0);
    match repo.find_user(user_id).await {
        Some(user) => Ok(user),
        None => Err(format!("User with ID {} not found", user_id)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_get_user_found() {
        // Create a mock repository
        let mut repo = UserRepositoryMock::new();
        repo.expect_find_user()
            .with(eq(1))
            .returning(|_| {
                Some(User {
                    id: 1,
                    name: "John Doe".to_string(),
                    email: "john@example.com".to_string(),
                })
            });
        
        // Create a mock param
        let mut params = HashMap::new();
        params.insert("id".to_string(), "1".to_string());
        let param = Param::from(params);
        
        // Call the handler
        let result = get_user(param, repo).await;
        
        // Assert on the result
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
    }
}
```

## Test Coverage

To measure test coverage, you can use tools like `cargo-tarpaulin`:

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

This will generate an HTML report showing which parts of your code are covered by tests.

For more advanced testing techniques, check out the [examples](https://github.com/ngyn-rs/ngyn/tree/main/examples) in the Ngyn repository.