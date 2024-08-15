use bytes::{Buf, Bytes};
use serde::Serialize;
use serde_json::{json, Value};
use std::any::{Any, TypeId};

use super::JsonResponse;

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

impl<T: Serialize> ToBytes for Vec<T> {
    fn to_bytes(self) -> Bytes {
        let json = json!(self);
        json.to_bytes()
    }
}

impl<D: Serialize, E: Serialize> ToBytes for JsonResponse<D, E> {
    fn to_bytes(self) -> Bytes {
        if let Some(data) = self.data() {
            return json!({ "data": data }).to_bytes();
        }
        json!({ "error": self.error() }).to_bytes()
    }
}

/// Converts a `Result` into a `Bytes`
///
/// In Ngyn, a `Result` can be converted into a `Bytes` object.
impl<T, E> ToBytes for Result<T, E>
where
    T: ToBytes + Any,
    E: ToBytes + Any,
{
    fn to_bytes(self) -> Bytes {
        if TypeId::of::<E>() == TypeId::of::<Value>() {
            // This is likely a JsonResult
            match self {
                Ok(value) => {
                    JsonResponse::<Value, Value>::new(Some(json!(value.to_bytes().chunk())), None)
                        .to_bytes()
                }
                Err(error) => {
                    JsonResponse::<Value, Value>::new(None, Some(json!(error.to_bytes().chunk())))
                        .to_bytes()
                }
            }
        } else {
            // This is a regular Result
            match self {
                Ok(value) => value.to_bytes(),
                Err(error) => error.to_bytes(),
            }
        }
    }
}
