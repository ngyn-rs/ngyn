use hyper::body::Bytes;
use serde::Serialize;
use serde_json::{json, Value};
use std::str::FromStr;

/// `ToBytes` can be used to convert a type into a `Bytes`
///
/// # Examples
///
/// ```rust ignore
/// use ngyn_shared::{ToBytes, Bytes};
///
/// let string_body: Bytes = "Hello, world!".to_string().to_bytes();
/// ```
pub trait ToBytes {
    /// Parses the body into a `Bytes` object.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::{ToBytes, Bytes};
    ///
    /// let bytes: Bytes = Bytes::from("Hello, world!");
    /// let parsed_bytes: Bytes = bytes.to_bytes();
    /// ```
    fn to_bytes(self) -> Bytes;
}

/// `ParseBytes` can be used to parse `Bytes` into a specific type
pub trait ParseBytes {
    /// Parses `Bytes` into a specific type
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::{ParseBytes, Bytes};
    ///
    /// let bytes: Bytes = Bytes::from("42");
    /// let value: i32 = bytes.parse_bytes();
    /// assert_eq!(value, 42);
    /// ```
    fn parse_bytes<T: FromStr + Default>(self) -> T;
}

impl ParseBytes for Bytes {
    fn parse_bytes<T: FromStr + Default>(self) -> T {
        String::from_utf8_lossy(&self)
            .parse::<T>()
            .unwrap_or_default()
    }
}

impl ToBytes for &str {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ToBytes for String {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self)
    }
}

impl ToBytes for Bytes {
    fn to_bytes(self) -> Bytes {
        self
    }
}

impl ToBytes for i32 {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ToBytes for i64 {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ToBytes for f32 {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ToBytes for f64 {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ToBytes for u32 {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ToBytes for u64 {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ToBytes for bool {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ToBytes for Value {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl<T: Serialize, E: Serialize> ToBytes for Result<T, E> {
    fn to_bytes(self) -> Bytes {
        match self {
            Ok(data) => json!({ "data": data }).to_bytes(),
            Err(error) => json!({ "error": error }).to_bytes(),
        }
    }
}

impl<T: Serialize> ToBytes for Vec<T> {
    fn to_bytes(self) -> Bytes {
        let json = json!(self);
        json.to_bytes()
    }
}
