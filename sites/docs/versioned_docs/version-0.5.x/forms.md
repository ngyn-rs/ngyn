---
sidebar_position: 2
---

# Forms Handling

Ngyn provides a simple way to handle form data in your applications. This guide will show you how to parse and validate form data in your route handlers.

## Parsing Form Data

To parse form data in your route handlers, you can use the `Body` struct. The `Body` struct provides methods to access form data sent in the request body.

Here's an example of a route handler that parses form data:

```rust
use ngyn::prelude::*;

#[handler]
async fn handle_form(body: Body) -> JsonResult {
    let mut name = None
    let mut email = None;
    let mut data = body.form_data().unwrap();

    while let Ok(Some(field)) = data.next_field().await {
        let key = field.name().unwrap();
        let value = field.text().await.unwrap();
        
        if key == "name" {
            name = Some(value);
        } else if key == "email" {
            email = Some(value);
        }
    }
    
    Ok(json!({
        "name": name,
        "email": email,
    }))
}
```