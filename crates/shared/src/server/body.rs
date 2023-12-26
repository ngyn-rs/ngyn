use std::collections::HashMap;

/// `IntoNgynBody` can be used to convert a type into a `NgynBody
///
/// # Examples
///
/// ```
/// use ngyn_shared::{IntoNgynBody, NgynBody};
///
/// let string_body: NgynBody = "Hello, world!".parse_body();
/// ```
pub trait IntoNgynBody {
    /// Converts the type into a `NgynBody`
    fn parse_body(self) -> NgynBody;
    fn from_body(body: NgynBody) -> Self;
}

#[derive(Clone, Debug, PartialEq)]
/// Represents the body of a response in Ngyn
///
/// # Examples
///
/// ```
/// use ngyn_shared::NgynBody;
///
/// let string_body = NgynBody::String("Hello, world!".to_string());
/// let bool_body = NgynBody::Bool(true);
/// let number_body = NgynBody::Number(42);
/// let none_body = NgynBody::None;
/// ```
pub enum NgynBody {
    /// Represents a string body
    String(String),
    /// Represents a boolean body
    Bool(bool),
    /// Represents a numeric body
    Number(usize),
    /// Represents a struct body
    Map(HashMap<String, NgynBody>),
    /// Represents no body
    None,
}

impl NgynBody {
    pub fn parse<T: IntoNgynBody>(self) -> T {
        T::from_body(self)
    }
}

impl IntoNgynBody for NgynBody {
    fn parse_body(self) -> NgynBody {
        self
    }
    fn from_body(body: NgynBody) -> Self {
        body
    }
}

impl IntoNgynBody for String {
    fn parse_body(self) -> NgynBody {
        NgynBody::String(self)
    }

    fn from_body(body: NgynBody) -> Self {
        match body {
            NgynBody::String(value) => value,
            _ => panic!("Cannot convert {:?} to String", body),
        }
    }
}

impl IntoNgynBody for bool {
    fn parse_body(self) -> NgynBody {
        NgynBody::Bool(self)
    }

    fn from_body(body: NgynBody) -> Self {
        match body {
            NgynBody::Bool(value) => value,
            _ => panic!("Cannot convert {:?} to bool", body),
        }
    }
}

impl IntoNgynBody for usize {
    fn parse_body(self) -> NgynBody {
        NgynBody::Number(self)
    }

    fn from_body(body: NgynBody) -> Self {
        match body {
            NgynBody::Number(value) => value,
            _ => panic!("Cannot convert {:?} to usize", body),
        }
    }
}

impl IntoNgynBody for i32 {
    fn parse_body(self) -> NgynBody {
        NgynBody::Number(self as usize)
    }

    fn from_body(body: NgynBody) -> Self {
        match body {
            NgynBody::Number(value) => value as i32,
            _ => panic!("Cannot convert {:?} to i32", body),
        }
    }
}

impl IntoNgynBody for f32 {
    fn parse_body(self) -> NgynBody {
        NgynBody::String(self.to_string())
    }

    fn from_body(body: NgynBody) -> Self {
        match body {
            NgynBody::String(value) => value.parse::<f32>().unwrap(),
            _ => panic!("Cannot convert {:?} to f32", body),
        }
    }
}

impl IntoNgynBody for Vec<u8> {
    fn parse_body(self) -> NgynBody {
        NgynBody::String(String::from_utf8_lossy(&self).to_string())
    }

    fn from_body(body: NgynBody) -> Self {
        match body {
            NgynBody::String(value) => value.into_bytes(),
            _ => panic!("Cannot convert {:?} to Vec<u8>", body),
        }
    }
}

impl IntoNgynBody for i64 {
    fn parse_body(self) -> NgynBody {
        NgynBody::Number(self as usize)
    }

    fn from_body(body: NgynBody) -> Self {
        match body {
            NgynBody::Number(value) => value as i64,
            _ => panic!("Cannot convert {:?} to i64", body),
        }
    }
}

impl IntoNgynBody for f64 {
    fn parse_body(self) -> NgynBody {
        NgynBody::String(self.to_string())
    }

    fn from_body(body: NgynBody) -> Self {
        match body {
            NgynBody::String(value) => value.parse::<f64>().unwrap(),
            _ => panic!("Cannot convert {:?} to f64", body),
        }
    }
}

impl IntoNgynBody for char {
    fn parse_body(self) -> NgynBody {
        NgynBody::String(self.to_string())
    }

    fn from_body(body: NgynBody) -> Self {
        match body {
            NgynBody::String(value) => value.chars().next().unwrap(),
            _ => panic!("Cannot convert {:?} to char", body),
        }
    }
}

impl IntoNgynBody for () {
    fn parse_body(self) -> NgynBody {
        NgynBody::None
    }

    fn from_body(body: NgynBody) -> Self {
        match body {
            NgynBody::None => (),
            _ => panic!("Cannot convert {:?} to ()", body),
        }
    }
}
