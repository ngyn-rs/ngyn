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
    /// Converts the type into a `Bytes`
    fn parse_body(self) -> Bytes;
}

pub trait ParseBytes {
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
