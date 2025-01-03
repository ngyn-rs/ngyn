---
sidebar_position: 2
---

# Forms Handling

Ngyn provides a simple way to handle form data in your applications. This guide will show you how to parse and validate form data in your route handlers.

## Parsing Form Data

To parse form data in your route handlers, you can use the `FormFields` struct. The `FormFields` struct provides methods to access form data sent in the request body.

Here's an example of a route handler that parses form data:

```rust
use ngyn::prelude::*;

#[handler]
async fn handle_form(fields: FormFields<'_cx_lifetime>) -> JsonResult {
    let mut fields = fields.await;
    
    if let Some(name) = fields.remove("name") {
        Ok(json!({ "name": name }))
    } else {
        Err("Name field is required".into())
    }
}
```