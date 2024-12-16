---
sidebar_position: 1
---

# Data Transfer Objects (DTOs)
Data Transfer Objects (DTOs) are objects that carry data between processes. They are used to encapsulate data and send it from one part of your app to another. DTOs can be used to reduce the number of method calls and to improve performance.

In ngyn, DTOs have a couple of super powers:
- They can be used to validate data before it is handled by routes.
- They can be used to transform data before it is handled by routes.
- They can be used to serialize data before it is sent to the client (valid responses)

## Creating a DTO
To create a DTO, you need to create a serializable struct that derives the `Dto` derive macro. The `Dto` derive macro is a custom derive that implements the `Dto` trait for the struct.

```rust
use ngyn::prelude::*;

#[derive(Dto, serde::Serialize, serde::Deserialize)]
struct CreateUserDto {
    username: String,
    email: String,
    password: String,
}
```

## Validating a DTO
In ngyn, you can either validate the data of a Dto through serde or by enabling the `validate` feature in your `Cargo.toml` file and implementing the `Validate` trait for the struct. Advanced usage of the `Validate` trait is covered in the [Validation](/docs/advanced/validation) section.

```rust examole validation with serde
use ngyn::prelude::*;

#[derive(Dto, serde::Serialize, serde::Deserialize)]
struct CreateUserDto {
    #[serde(validate(length(min = 3, max = 20))]
    username: String,
    #[serde(validate(email))]
    email: String,
    #[serde(validate(length(min = 8))]
    password: String,
}
```

Validating using serde has a significant limitation. Errors aren't returned as a result of the validation. Instead, the validation errors are returned as a `500 Internal Server Error` response. To get around this limitation, you can use the `validate` feature.


## Using a DTO
To use a DTO, you can simply create an instance of the struct and pass it to a route handler.

```rust
use ngyn::prelude::*;

#[derive(Dto, serde::Serialize, serde::Deserialize)]
struct CreateUserDto {
    username: String,
    email: String,
    password: String,
}

#[controller]
struct UserController;

#[routes]
impl UserController {
    #[post("/users")]
    async fn create_user(dto: CreateUserDto) -> Result<u16, Error> {
        // Create a user
        Ok(201)
    }
}
```
