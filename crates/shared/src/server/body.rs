use hyper::body::Bytes;
use std::str::FromStr;

/// `ToBytes` can be used to convert a type into a `Bytes`
///
/// # Examples
///
/// ```
/// use ngyn_shared::{ToBytes, Bytes};
///
/// let string_body: Bytes = "Hello, world!".to_string().to_bytes();
/// ```
pub trait ToBytes {
    /// Parses the body into a `Bytes` object.
    ///
    /// # Examples
    ///
    /// ```
    /// use ngyn_shared::{ToBytes, Bytes};
    ///
    /// let bytes: Bytes = Bytes::from("Hello, world!");
    /// let parsed_bytes: Bytes = bytes.to_bytes();
    /// ```
    fn to_bytes(self) -> Bytes;
}

/// `FromBytes` can be used to parse `Bytes` into a specific type
pub trait FromBytes {
    /// Parses `Bytes` into a specific type
    ///
    /// # Examples
    ///
    /// ```
    /// use ngyn_shared::{FromBytes, Bytes};
    ///
    /// let bytes: Bytes = Bytes::from("42");
    /// let value: i32 = bytes.from_bytes();
    /// assert_eq!(value, 42);
    /// ```
    fn from_bytes<T: FromStr + Default>(self) -> T;
}

impl FromBytes for Bytes {
    fn from_bytes<T: FromStr + Default>(self) -> T {
        String::from_utf8_lossy(&self)
            .parse::<T>()
            .unwrap_or_default()
    }
}

impl ToBytes for Bytes {
    fn to_bytes(self) -> Bytes {
        self
    }
}

impl ToBytes for String {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self)
    }
}

impl ToBytes for bool {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ToBytes for usize {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ToBytes for i32 {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ToBytes for f32 {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ToBytes for Vec<u8> {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self)
    }
}

impl ToBytes for i64 {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ToBytes for f64 {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ToBytes for char {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

impl ToBytes for () {
    fn to_bytes(self) -> Bytes {
        Bytes::default()
    }
}
