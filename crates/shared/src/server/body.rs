use hyper::body::Bytes;
use std::str::FromStr;

/// `ParseBody` can be used to convert a type into a `Bytes`
///
/// # Examples
///
/// ```
/// use ngyn_shared::{ParseBody, Bytes};
///
/// let string_body: Bytes = "Hello, world!".to_string().parse_body();
/// ```
pub trait ParseBody {
    /// Parses the body into a `Bytes` object.
    ///
    /// # Examples
    ///
    /// ```
    /// use ngyn_shared::{ParseBody, Bytes};
    ///
    /// let bytes: Bytes = Bytes::from("Hello, world!");
    /// let parsed_bytes: Bytes = bytes.parse_body();
    /// ```
    fn parse_body(self) -> Bytes;
}

/// `ParseBytes` can be used to parse `Bytes` into a specific type
pub trait ParseBytes {
    /// Parses `Bytes` into a specific type
    ///
    /// # Examples
    ///
    /// ```
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

impl ParseBody for Bytes {
    fn parse_body(self) -> Bytes {
        self
    }
}

impl ParseBody for String {
    fn parse_body(self) -> Bytes {
        Bytes::from(self)
    }
}

impl ParseBody for bool {
    fn parse_body(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ParseBody for usize {
    fn parse_body(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ParseBody for i32 {
    fn parse_body(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ParseBody for f32 {
    fn parse_body(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ParseBody for Vec<u8> {
    fn parse_body(self) -> Bytes {
        Bytes::from(self)
    }
}

impl ParseBody for i64 {
    fn parse_body(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ParseBody for f64 {
    fn parse_body(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ParseBody for char {
    fn parse_body(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ParseBody for () {
    fn parse_body(self) -> Bytes {
        Bytes::default()
    }
}

