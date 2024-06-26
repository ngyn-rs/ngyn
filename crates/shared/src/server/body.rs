use hyper::body::Bytes;
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

impl<T: ToString> ToBytes for T {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}
