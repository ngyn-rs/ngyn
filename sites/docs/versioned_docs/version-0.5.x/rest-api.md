---
sidebar_position: 5
---

# Building RESTful APIs

RESTful APIs are a common way to expose your application's functionality to other services or clients. Ngyn provides a clean and intuitive way to build RESTful APIs. This guide will show you how to create a RESTful API using Ngyn.

## RESTful API Basics

REST (Representational State Transfer) is an architectural style for designing networked applications. RESTful APIs typically use HTTP methods explicitly and have a consistent URL structure.

The common HTTP methods used in RESTful APIs are:

- **GET**: Retrieve a resource or collection of resources
- **POST**: Create a new resource
- **PUT**: Update an existing resource (full update)
- **PATCH**: Partially update an existing resource
- **DELETE**: Remove a resource

## Setting Up a Basic RESTful API

Let's create a simple RESTful API for managing a collection of items:

```rust
use ngyn::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Define our data model
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Item {
    id: usize,
    name: String,
    description: String,
}

// In-memory store for our items
type ItemStore = Arc<Mutex<Vec<Item>>>;

// Handler to get all items
#[handler]
fn get_items(store: ItemStore) -> JsonResult {
    let items = store.lock().unwrap();
    Ok(json!({ "items": *items }))
}

// Handler to get a single item by ID
#[handler]
fn get_item(param: Param, store: ItemStore) -> Result<JsonResult, String> {
    let id = param.get("id")
        .unwrap_or("0")
        .parse::<usize>()
        .map_err(|_| "Invalid ID format".to_string())?;
    
    let items = store.lock().unwrap();
    
    if let Some(item) = items.iter().find(|item| item.id == id) {
        Ok(Ok(json!(item)))
    } else {
        Err(format!("Item with ID {} not found", id))
    }
}

// Handler to create a new item
#[handler]
async fn create_item(body: Body, store: ItemStore) -> Result<JsonResult, String> {
    let mut new_item = match body.json::<Item>().await {
        Ok(item) => item,
        Err(e) => return Err(format!("Invalid request body: {}", e)),
    };
    
    let mut items = store.lock().unwrap();
    
    // Generate a new ID (in a real app, this would be handled by the database)
    new_item.id = items.len() + 1;
    
    // Add the new item to our store
    items.push(new_item.clone());
    
    Ok(Ok(json!({
        "message": "Item created successfully",
        "item": new_item
    })))
}

// Handler to update an existing item
#[handler]
async fn update_item(param: Param, body: Body, store: ItemStore) -> Result<JsonResult, String> {
    let id = param.get("id")
        .unwrap_or("0")
        .parse::<usize>()
        .map_err(|_| "Invalid ID format".to_string())?;
    
    let updated_item = match body.json::<Item>().await {
        Ok(item) => item,
        Err(e) => return Err(format!("Invalid request body: {}", e)),
    };
    
    let mut items = store.lock().unwrap();
    
    if let Some(item) = items.iter_mut().find(|item| item.id == id) {
        // Update the item (preserving the ID)
        item.name = updated_item.name;
        item.description = updated_item.description;
        
        Ok(Ok(json!({
            "message": "Item updated successfully",
            "item": item
        })))
    } else {
        Err(format!("Item with ID {} not found", id))
    }
}

// Handler to delete an item
#[handler]
fn delete_item(param: Param, store: ItemStore) -> Result<JsonResult, String> {
    let id = param.get("id")
        .unwrap_or("0")
        .parse::<usize>()
        .map_err(|_| "Invalid ID format".to_string())?;
    
    let mut items = store.lock().unwrap();
    
    let initial_len = items.len();
    items.retain(|item| item.id != id);
    
    if items.len() < initial_len {
        Ok(Ok(json!({
            "message": "Item deleted successfully"
        })))
    } else {
        Err(format!("Item with ID {} not found", id))
    }
}

#[tokio::main]
async fn main() {
    // Create our in-memory store
    let store = Arc::new(Mutex::new(Vec::<Item>::new()));
    
    let mut app = HyperApplication::default();
    
    // Register our API routes
    app.group("/api/items", |group| {
        group.get("", get_items.with(store.clone()));          // GET /api/items
        group.get("/{id}", get_item.with(store.clone()));     // GET /api/items/1
        group.post("", create_item.with(store.clone()));       // POST /api/items
        group.put("/{id}", update_item.with(store.clone()));  // PUT /api/items/1
        group.delete("/{id}", delete_item.with(store.clone())); // DELETE /api/items/1
    });
    
    println!("RESTful API running at http://127.0.0.1:3000");
    let _ = app.listen("127.0.0.1:3000").await;
}
```

## API Versioning

As your API evolves, you might need to introduce breaking changes. API versioning allows you to maintain backward compatibility while introducing new features.

Here's how you can implement API versioning in Ngyn:

```rust
let mut app = HyperApplication::default();

// Version 1 of the API
app.group("/api/v1", |v1| {
    v1.group("/items", |items| {
        items.get("", get_items_v1.with(store.clone()));
        // Other v1 routes...
    });
});

// Version 2 of the API
app.group("/api/v2", |v2| {
    v2.group("/items", |items| {
        items.get("", get_items_v2.with(store.clone()));
        // Other v2 routes...
    });
});
```

## Request Validation

Validating incoming requests is crucial for building robust APIs. You can implement validation in your handlers:

```rust
#[derive(Deserialize)]
struct CreateItemRequest {
    name: String,
    description: String,
}

#[handler]
async fn create_item(body: Body, store: ItemStore) -> Result<JsonResult, String> {
    let req = match body.json::<CreateItemRequest>().await {
        Ok(req) => req,
        Err(e) => return Err(format!("Invalid request body: {}", e)),
    };
    
    // Validate the request
    if req.name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    
    if req.description.is_empty() {
        return Err("Description cannot be empty".to_string());
    }
    
    // Create the item
    let new_item = Item {
        id: 0, // Will be set later
        name: req.name,
        description: req.description,
    };
    
    // Rest of the handler...
    // ...
}
```

## Pagination

For endpoints that return collections, it's a good practice to implement pagination:

```rust
#[handler]
fn get_items(query: Query, store: ItemStore) -> JsonResult {
    let page = query.get("page")
        .unwrap_or("1")
        .parse::<usize>()
        .unwrap_or(1);
    
    let per_page = query.get("per_page")
        .unwrap_or("10")
        .parse::<usize>()
        .unwrap_or(10);
    
    let items = store.lock().unwrap();
    
    let start = (page - 1) * per_page;
    let end = std::cmp::min(start + per_page, items.len());
    
    let paginated_items = if start < items.len() {
        items[start..end].to_vec()
    } else {
        Vec::new()
    };
    
    Ok(json!({
        "items": paginated_items,
        "pagination": {
            "total": items.len(),
            "page": page,
            "per_page": per_page,
            "total_pages": (items.len() + per_page - 1) / per_page
        }
    }))
}
```

## Error Handling

Consistent error handling is important for a good API experience. Here's a more structured approach to error handling:

```rust
#[derive(Debug, Serialize)]
struct ApiError {
    code: String,
    message: String,
}

impl ApiError {
    fn not_found(resource: &str, id: &str) -> Self {
        Self {
            code: "NOT_FOUND".to_string(),
            message: format!("{} with ID {} not found", resource, id),
        }
    }
    
    fn validation(message: &str) -> Self {
        Self {
            code: "VALIDATION_ERROR".to_string(),
            message: message.to_string(),
        }
    }
    
    fn internal(message: &str) -> Self {
        Self {
            code: "INTERNAL_ERROR".to_string(),
            message: message.to_string(),
        }
    }
}

impl From<ApiError> for NgynResponse {
    fn from(error: ApiError) -> Self {
        let status = match error.code.as_str() {
            "NOT_FOUND" => http::StatusCode::NOT_FOUND,
            "VALIDATION_ERROR" => http::StatusCode::BAD_REQUEST,
            _ => http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        
        let body = Body::from(serde_json::to_string(&json!({
            "error": error
        })).unwrap());
        
        let mut response = NgynResponse::new(body);
        *response.status_mut() = status;
        response.headers_mut().insert(
            "Content-Type", 
            "application/json".parse().unwrap()
        );
        
        response
    }
}

// Use in handlers
#[handler]
fn get_item(param: Param, store: ItemStore) -> Result<JsonResult, ApiError> {
    let id = param.get("id")
        .unwrap_or("0")
        .parse::<usize>()
        .map_err(|_| ApiError::validation("Invalid ID format"))?;
    
    let items = store.lock().unwrap();
    
    if let Some(item) = items.iter().find(|item| item.id == id) {
        Ok(Ok(json!(item)))
    } else {
        Err(ApiError::not_found("Item", &id.to_string()))
    }
}
```

## CORS Support

Cross-Origin Resource Sharing (CORS) is important if your API will be accessed from web browsers on different domains:

```rust
struct CorsMiddleware;

impl NgynMiddleware for CorsMiddleware {
    async fn handle(ctx: NgynContext) {
        let headers = ctx.response_mut().headers_mut();
        
        headers.insert(
            "Access-Control-Allow-Origin", 
            "*".parse().unwrap()
        );
        
        headers.insert(
            "Access-Control-Allow-Methods", 
            "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap()
        );
        
        headers.insert(
            "Access-Control-Allow-Headers", 
            "Content-Type, Authorization".parse().unwrap()
        );
    }
}

// Add the middleware to your application
app.use_middleware(CorsMiddleware {});

// Handle OPTIONS requests for CORS preflight
app.options("*", handler(|_| ""));
```

## Authentication

Most APIs require some form of authentication. Here's a simple example using JWT (JSON Web Tokens):

```rust
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // Subject (user ID)
    exp: usize,   // Expiration time
    role: String, // User role
}

struct JwtAuthGate;

impl NgynGate for JwtAuthGate {
    async fn can_activate(ctx: NgynContext) -> bool {
        let auth_header = match ctx.request().headers().get("Authorization") {
            Some(header) => header,
            None => {
                *ctx.response_mut().status_mut() = http::StatusCode::UNAUTHORIZED;
                return false;
            }
        };
        
        let auth_str = match auth_header.to_str() {
            Ok(str) => str,
            Err(_) => {
                *ctx.response_mut().status_mut() = http::StatusCode::UNAUTHORIZED;
                return false;
            }
        };
        
        if !auth_str.starts_with("Bearer ") {
            *ctx.response_mut().status_mut() = http::StatusCode::UNAUTHORIZED;
            return false;
        }
        
        let token = &auth_str[7..]; // Remove "Bearer " prefix
        
        let secret = "your-secret-key"; // In a real app, this would be an environment variable
        
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS256)
        ) {
            Ok(token_data) => {
                // Store the claims in the context for later use
                ctx.set("user_id", token_data.claims.sub);
                ctx.set("user_role", token_data.claims.role);
                true
            },
            Err(_) => {
                *ctx.response_mut().status_mut() = http::StatusCode::UNAUTHORIZED;
                false
            }
        }
    }
}

// Use the gate to protect routes
#[handler(gates = [JwtAuthGate])]
fn protected_route(ctx: NgynContext) -> JsonResult {
    let user_id = ctx.get::<String>("user_id").unwrap_or_default();
    let user_role = ctx.get::<String>("user_role").unwrap_or_default();
    
    Ok(json!({
        "message": "This is a protected route",
        "user_id": user_id,
        "user_role": user_role
    }))
}
```

## Rate Limiting

To protect your API from abuse, you can implement rate limiting:

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

struct RateLimiter {
    // Map of IP address to (request count, last request time)
    clients: Arc<Mutex<HashMap<String, (usize, Instant)>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    fn new(max_requests: usize, window_seconds: u64) -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window: Duration::from_secs(window_seconds),
        }
    }
}

impl NgynMiddleware for RateLimiter {
    async fn handle(ctx: NgynContext) {
        // Get the client IP address
        let ip = match ctx.request().headers().get("X-Forwarded-For") {
            Some(header) => header.to_str().unwrap_or("unknown").to_string(),
            None => "127.0.0.1".to_string(), // Default for local testing
        };
        
        let now = Instant::now();
        let mut is_rate_limited = false;
        
        // Update rate limiting information
        {
            let mut clients = self.clients.lock().unwrap();
            
            if let Some((count, last_request_time)) = clients.get_mut(&ip) {
                if now.duration_since(*last_request_time) > self.window {
                    // Reset if outside the window
                    *count = 1;
                    *last_request_time = now;
                } else {
                    // Increment count
                    *count += 1;
                    *last_request_time = now;
                    
                    if *count > self.max_requests {
                        is_rate_limited = true;
                    }
                }
            } else {
                // First request from this IP
                clients.insert(ip, (1, now));
            }
        }
        
        if is_rate_limited {
            *ctx.response_mut().status_mut() = http::StatusCode::TOO_MANY_REQUESTS;
            ctx.response_mut().headers_mut().insert(
                "Retry-After", 
                "60".parse().unwrap()
            );
        }
    }
}

// Add the rate limiter to your application
app.use_middleware(RateLimiter::new(100, 60)); // 100 requests per minute
```

## Best Practices

Here are some best practices for building RESTful APIs with Ngyn:

1. **Use HTTP methods correctly**: GET for reading, POST for creating, PUT for updating, DELETE for removing resources.

2. **Use appropriate status codes**: 200 for success, 201 for creation, 400 for bad requests, 401 for unauthorized, 404 for not found, etc.

3. **Implement pagination** for endpoints that return collections.

4. **Version your API** to maintain backward compatibility.

5. **Validate input** to prevent invalid data and potential security issues.

6. **Use consistent error responses** to make error handling easier for API consumers.

7. **Implement rate limiting** to protect your API from abuse.

8. **Document your API** using tools like Swagger.

9. **Use HTTPS** in production to secure data transmission.

10. **Implement proper authentication and authorization** to protect sensitive resources.

## API Security Best Practices

Security is a critical aspect of any API. Here are some additional security best practices to consider when building your Ngyn RESTful APIs:

### Input Sanitization

Always sanitize and validate all input data to prevent injection attacks:

```rust
// Example of input sanitization for a search parameter
#[handler]
fn search_items(query: Query, store: ItemStore) -> JsonResult {
    let search_term = query.get("q").unwrap_or("");
    
    // Sanitize the search term (example: limit length and remove special characters)
    let search_term = if search_term.len() > 100 {
        &search_term[0..100]
    } else {
        search_term
    };
    
    // Use a regex to remove potentially dangerous characters
    let re = Regex::new(r"[^\w\s]").unwrap();
    let sanitized_term = re.replace_all(search_term, "").to_string();
    
    // Now use the sanitized term for your search logic
    // ...
    
    Ok(json!({ "results": [] }))
}
```

### API Keys and Secrets Management

Never hardcode API keys or secrets in your application code. Use environment variables or a secure vault:

```rust
use std::env;

#[handler]
async fn external_api_call() -> JsonResult {
    // Get API key from environment variable
    let api_key = env::var("EXTERNAL_API_KEY")
        .expect("EXTERNAL_API_KEY must be set");
    
    // Use the API key for your external API call
    // ...
    
    Ok(json!({ "status": "success" }))
}
```

### Request Throttling

Implement more sophisticated request throttling based on user identity or API key:

```rust
struct ApiKeyRateLimiter {
    // Map of API key to (request count, last request time)
    clients: Arc<Mutex<HashMap<String, (usize, Instant)>>>,
    max_requests: usize,
    window: Duration,
}

impl NgynMiddleware for ApiKeyRateLimiter {
    async fn handle(ctx: NgynContext) {
        // Extract API key from request header or query parameter
        let api_key = ctx.request().headers().get("X-API-Key")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("anonymous");
        
        // Apply rate limiting logic based on API key
        // ...
    }
}
```

## Testing RESTful APIs

Testing is essential for ensuring your API works correctly and remains stable over time. Here are some approaches for testing Ngyn RESTful APIs:

### Unit Testing Handlers

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use ngyn::test::TestClient;
    
    #[tokio::test]
    async fn test_get_items() {
        // Create a test store with some items
        let store = Arc::new(Mutex::new(vec![
            Item { id: 1, name: "Test Item 1".to_string(), description: "Description 1".to_string() },
            Item { id: 2, name: "Test Item 2".to_string(), description: "Description 2".to_string() },
        ]));
        
        // Create a test client
        let mut app = HyperApplication::default();
        app.get("/api/items", get_items.with(store.clone()));
        let client = TestClient::new(app);
        
        // Make a test request
        let response = client.get("/api/items").send().await;
        
        // Assert the response
        assert_eq!(response.status(), 200);
        
        // Parse and verify the response body
        let body: serde_json::Value = response.json().await.unwrap();
        let items = body["items"].as_array().unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0]["id"], 1);
        assert_eq!(items[1]["name"], "Test Item 2");
    }
}
```

### Integration Testing

For integration tests, you can start your actual application and make HTTP requests to it:

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use reqwest;
    use tokio::task;
    
    #[tokio::test]
    async fn test_full_api() {
        // Start the application in a separate task
        let server_handle = task::spawn(async {
            let mut app = create_application();
            app.listen("127.0.0.1:3001").await
        });
        
        // Give the server a moment to start
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        
        // Create a client and make requests
        let client = reqwest::Client::new();
        
        // Test creating an item
        let create_response = client.post("http://127.0.0.1:3001/api/items")
            .json(&serde_json::json!({
                "name": "New Item",
                "description": "Integration test item"
            }))
            .send()
            .await
            .unwrap();
        
        assert_eq!(create_response.status(), 200);
        
        // Test getting the created item
        let get_response = client.get("http://127.0.0.1:3001/api/items")
            .send()
            .await
            .unwrap();
        
        assert_eq!(get_response.status(), 200);
        
        // Clean up
        // ...
    }
}
```

## Performance Optimization

As your API grows, performance becomes increasingly important. Here are some tips for optimizing your Ngyn RESTful APIs:

### Caching

Implement caching for frequently accessed resources:

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

struct CacheEntry<T> {
    data: T,
    expires_at: Instant,
}

struct SimpleCache<T> {
    entries: Arc<Mutex<HashMap<String, CacheEntry<T>>>>,
    ttl: Duration,
}

impl<T: Clone> SimpleCache<T> {
    fn new(ttl_seconds: u64) -> Self {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }
    
    fn get(&self, key: &str) -> Option<T> {
        let mut entries = self.entries.lock().unwrap();
        
        if let Some(entry) = entries.get(key) {
            if Instant::now() < entry.expires_at {
                return Some(entry.data.clone());
            } else {
                // Remove expired entry
                entries.remove(key);
            }
        }
        
        None
    }
    
    fn set(&self, key: String, data: T) {
        let mut entries = self.entries.lock().unwrap();
        
        entries.insert(key, CacheEntry {
            data,
            expires_at: Instant::now() + self.ttl,
        });
    }
}

// Use in a handler
#[handler]
async fn get_cached_items(cache: SimpleCache<Vec<Item>>, store: ItemStore) -> JsonResult {
    // Try to get from cache first
    if let Some(items) = cache.get("all_items") {
        return Ok(json!({ "items": items, "source": "cache" }));
    }
    
    // If not in cache, get from store
    let items = store.lock().unwrap().clone();
    
    // Update cache
    cache.set("all_items".to_string(), items.clone());
    
    Ok(json!({ "items": items, "source": "store" }))
}
```

### Asynchronous Processing

For time-consuming operations, consider using asynchronous processing:

```rust
#[handler]
async fn process_large_dataset(body: Body) -> JsonResult {
    // Start a background task
    tokio::spawn(async move {
        // Perform time-consuming operation
        // ...
    });
    
    // Return immediately with a job ID
    Ok(json!({
        "status": "processing",
        "job_id": "abc123",
        "message": "Your request is being processed"
    }))
}
```


For more advanced API examples, check out the [examples](https://github.com/ngyn-rs/ngyn/tree/main/examples) in the Ngyn repository.