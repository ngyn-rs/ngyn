use bytes::Bytes;
use http::HeaderMap;
use http_body_util::BodyExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::server::{NgynContext, NgynResponse, Transformer};

#[derive(Serialize, Deserialize)]
/// Responses are hard to manage, especially when they are not standardized.
/// This is why Ngyn, by default, provides a json response format.
///
/// The json response format is a JSON object with two keys: `data` and `error`.
/// This would ideally make your responses more predictable and easier to manage.
/// A valid response would look like:
/// ```json
/// {
///    "data": {
///       "key": "value"
///   },
///  "error": null
/// }
/// ```
/// A valid error response would look like:
/// ```json
/// {
///   "data": null,
///  "error": {
///    "status": 404,
///    "message": "Not Found"
///   }
/// }
/// ```
/// The `data` key is used to store the response data, while the `error` key is used to store error data.
/// Both keys are optional, but at least one of them should be present.
///
///
/// ### How to create a json response?
/// Ngyn provides an implementation on [`JsonResult`] to convert it to a json response.
/// This means anytime you make use of a `JsonResult` in your controlled routes, it will be converted to a json response.
///
/// #### Example
/// ```rust ignore
/// use ngyn::prelude::*;
///
/// #[controller]
/// struct MyController;
///
/// #[routes]
/// impl MyController {
///    #[get("/")]
///   async fn get(&self, cx: &mut NgynContext) -> Result<Vec<u8>, ()> {
///    let data = vec![1, 2, 3];
///    Ok(data)
///   }
/// }
/// ```
pub struct JsonResponse<D: Serialize, E: Serialize> {
    data: Option<D>,
    error: Option<E>,
}

impl<D: Serialize, E: Serialize> JsonResponse<D, E> {
    /// Creates a new json response.
    pub fn new(data: Option<D>, error: Option<E>) -> Self {
        Self { data, error }
    }

    /// Returns the data.
    pub fn data(&self) -> Option<&D> {
        self.data.as_ref()
    }

    /// Returns the error data.
    pub fn error(&self) -> Option<&E> {
        self.error.as_ref()
    }
}

/// A shorthand for a json result.
///
/// This is useful when you want to return a json response.
/// It is a type alias for a [`Result`] with a [`Value`] as the `ok` and `error` type.
///
/// ### Example
///
/// ```rust ignore
/// use ngyn::prelude::*;
///
/// #[controller]
/// struct MyController;
///
/// #[routes]
/// impl MyController {
///    #[get("/")]
///   async fn get(&self, cx: &mut NgynContext) -> JsonResult {
///     let data = json!({ "key": "value" });
///     Ok(data)
///   }
/// }
/// ```
pub type JsonResult = Result<Value, Value>;

impl<'a> Transformer<'a> for &'a NgynResponse {
    fn transform(_cx: &'a mut NgynContext, res: &'a mut NgynResponse) -> Self {
        res
    }
}

impl<'a> Transformer<'a> for &'a mut NgynResponse {
    fn transform(_cx: &'a mut NgynContext, res: &'a mut NgynResponse) -> Self {
        res
    }
}

/// A shorthand for transforming a `HeaderMap` reference.
///
/// This is useful when you need to access the headers of a response.
impl<'a> Transformer<'a> for &'a HeaderMap {
    fn transform(_cx: &'a mut NgynContext, res: &'a mut NgynResponse) -> Self {
        res.headers()
    }
}

/// A shorthand for transforming a mutable `HeaderMap` reference.
///
/// This is useful when you want to add or remove headers from a response.
impl<'a> Transformer<'a> for &'a mut HeaderMap {
    fn transform(_cx: &'a mut NgynContext, res: &'a mut NgynResponse) -> Self {
        res.headers_mut()
    }
}

pub trait ReadBytes {
    #[allow(async_fn_in_trait)]
    /// Reads the bytes of a valid ngyn response body.
    ///
    /// You can use this to read the bytes of a response body.
    async fn read_bytes(&mut self) -> Result<Bytes, Box<dyn std::error::Error>>;
}

impl ReadBytes for NgynResponse {
    async fn read_bytes(&mut self) -> Result<Bytes, Box<dyn std::error::Error>> {
        let frame = self.frame().await;
        if let Some(Ok(frame)) = frame {
            if let Ok(bytes) = frame.into_data() {
                return Ok(bytes);
            }
        }
        Err("No response bytes has been set".into())
    }
}

pub trait PeekBytes {
    #[allow(async_fn_in_trait)]
    /// Peeks the bytes of a valid ngyn response body.
    ///
    /// You can use this to read the bytes of a response body without consuming it(Well, we make it look like we don't).
    async fn peek_bytes(&mut self, f: impl FnMut(&Bytes));
}

impl PeekBytes for NgynResponse {
    async fn peek_bytes(&mut self, mut f: impl FnMut(&Bytes)) {
        let frame = self.frame().await;
        if let Some(Ok(frame)) = frame {
            if let Ok(bytes) = frame.into_data() {
                f(&bytes);
                // body has been read, so we need to set it back
                *self.body_mut() = bytes.into();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let response = JsonResponse::new(Some("data"), Some("error"));
        assert_eq!(response.data(), Some(&"data"));
        assert_eq!(response.error(), Some(&"error"));
    }

    #[test]
    fn test_data() {
        let response: JsonResponse<&str, &str> = JsonResponse::new(Some("data"), None);
        assert_eq!(response.data(), Some(&"data"));
        assert_eq!(response.error(), None);
    }

    #[test]
    fn test_error() {
        let response: JsonResponse<&str, &str> = JsonResponse::new(None, Some("error"));
        assert_eq!(response.data(), None);
        assert_eq!(response.error(), Some(&"error"));
    }

    #[tokio::test]
    async fn test_peek_bytes() {
        let mut response = NgynResponse::default();
        let body = Bytes::from("Hello, world!");
        *response.body_mut() = body.clone().into();

        let mut bytes = Vec::new();
        let peek_fn = |data: &Bytes| {
            bytes.extend_from_slice(&data);
        };

        response.peek_bytes(peek_fn).await;
        assert_eq!(bytes, body);
    }

    #[tokio::test]
    async fn test_read_bytes() {
        let mut response = NgynResponse::default();
        let body = Bytes::from("Hello, world!");
        *response.body_mut() = body.clone().into();

        let bytes = response.read_bytes().await.unwrap();
        assert_eq!(bytes, body);
    }
}
