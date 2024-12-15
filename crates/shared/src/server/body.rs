use bytes::Bytes;
use serde::Serialize;
use serde_json::{json, Value};
use std::any::{Any, TypeId};

use super::JsonResponse;

/// `ToBytes` can be used to convert a type into a `Bytes`
///
/// ### Examples
///
/// ```rust ignore
/// use ngyn_shared::{ToBytes, Bytes};
///
/// let string_body: Bytes = "Hello, world!".to_string().to_bytes();
/// ```
pub trait ToBytes {
    /// Parses the body into a `Bytes` object.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use ngyn_shared::{ToBytes, Bytes};
    ///
    /// let bytes: Bytes = Bytes::from("Hello, world!");
    /// let parsed_bytes: Bytes = bytes.to_bytes();
    /// ```
    fn to_bytes(self) -> Bytes;
}

impl ToBytes for () {
    fn to_bytes(self) -> Bytes {
        Bytes::default()
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

impl ToBytes for &String {
    fn to_bytes(self) -> Bytes {
        Bytes::from(self.to_string())
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
    T: ToBytes + Serialize + Any,
    E: ToBytes + Serialize + Any,
{
    fn to_bytes(self) -> Bytes {
        if TypeId::of::<E>() == TypeId::of::<Value>() && TypeId::of::<T>() == TypeId::of::<Value>()
        {
            // This is likely a JsonResult
            match self {
                Ok(value) => JsonResponse::<Value, Value>::new(Some(json!(value)), None).to_bytes(),
                Err(error) => {
                    JsonResponse::<Value, Value>::new(None, Some(json!(error))).to_bytes()
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

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn test_to_bytes_string() {
        let input = "Hello, world!";
        let expected = Bytes::from(input);
        let result = input.to_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_bytes_str() {
        let input = "Hello, world!";
        let expected = Bytes::from(input);
        let result = input.to_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_bytes_bytes() {
        let input = Bytes::from("Hello, world!");
        let expected = input.clone();
        let result = input.to_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_bytes_i32() {
        let input = 42;
        let expected = Bytes::from(input.to_string());
        let result = input.to_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_bytes_i64() {
        let input = 42;
        let expected = Bytes::from(input.to_string());
        let result = input.to_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_bytes_f32() {
        let expected = Bytes::from(PI.to_string());
        let result = PI.to_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_bytes_f64() {
        let expected = Bytes::from(PI.to_string());
        let result = PI.to_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_bytes_u32() {
        let input = 42;
        let expected = Bytes::from(input.to_string());
        let result = input.to_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_bytes_u64() {
        let input = 42;
        let expected = Bytes::from(input.to_string());
        let result = input.to_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_bytes_bool() {
        let input = true;
        let expected = Bytes::from(input.to_string());
        let result = input.to_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_bytes_value() {
        let input = json!({ "key": "value" });
        let expected = Bytes::from(input.to_string());
        let result = input.to_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_bytes_vec() {
        let input = vec![1, 2, 3];
        let expected = Bytes::from(json!(input).to_string());
        let result = input.to_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_bytes_json_response_with_data() {
        let data = json!({ "key": "value" });
        let response: JsonResponse<Value, Value> = JsonResponse::new(Some(data.clone()), None);
        let expected = Bytes::from(json!({ "data": data }).to_string());
        let result = response.to_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_bytes_json_response_with_error() {
        let error = json!({ "message": "Error occurred" });
        let response: JsonResponse<Value, Value> = JsonResponse::new(None, Some(error.clone()));
        let expected = Bytes::from(json!({ "error": error }).to_string());
        let result = response.to_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_bytes_result_json_result() {
        let value = json!({ "key": "value" });
        let result: Result<Value, Value> = Ok(value.clone());
        let expected = Bytes::from(json!({ "data": value }).to_string());
        let result_bytes = result.to_bytes();
        assert_eq!(result_bytes, expected);
    }

    #[test]
    fn test_to_bytes_result_json_result_error() {
        let error = json!({ "message": "Error occurred" });
        let result: Result<Value, Value> = Err(error.clone());
        let expected = Bytes::from(json!({ "error": error }).to_string());
        let result_bytes = result.to_bytes();
        assert_eq!(result_bytes, expected);
    }

    #[test]
    fn test_to_bytes_result_regular_result() {
        let value = "Hello, world!";
        let result: Result<&str, &str> = Ok(value);
        let expected = Bytes::from(value);
        let result_bytes = result.to_bytes();
        assert_eq!(result_bytes, expected);
    }

    #[test]
    fn test_to_bytes_result_regular_result_error() {
        let error = "Error occurred";
        let result: Result<&str, &str> = Err(error);
        let expected = Bytes::from(error);
        let result_bytes = result.to_bytes();
        assert_eq!(result_bytes, expected);
    }
}
