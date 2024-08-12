use hyper::{header::IntoHeaderName, StatusCode};
use serde::{Deserialize, Serialize};

use crate::server::{NgynContext, NgynResponse, ToBytes, Transformer};

#[derive(Serialize, Deserialize)]
pub struct CommonResponse<D, E> {
    pub data: Option<D>,
    pub error: Option<E>,
}

/// Trait representing a full response.
///
/// This trait provides short methods to set the status code, headers, and body of a response.
pub trait FullResponse {
    /// Sets the status code of the response.
    ///
    /// # Arguments
    ///
    /// * `status` - The status code to set.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use http_body_util::Full;
    /// use hyper::StatusCode;
    /// use crate::{context::NgynContext, transformer::Transformer, NgynResponse, ToBytes};
    ///
    /// struct MyResponse {
    ///     status: StatusCode,
    /// }
    ///
    /// let mut response = MyResponse { status: StatusCode::OK };
    /// response.set_status(200);
    /// assert_eq!(response.status, StatusCode::OK);
    /// ```
    fn set_status(&mut self, status: u16);

    /// Sets a header - name, value pair - of the response.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the header.
    /// * `value` - The value of the header.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// let mut response = NgynResponse::default();
    /// response.set_header("Content-Type", "application/json");
    /// assert_eq!(response.headers.get("Content-Type"), Some(&"application/json".to_string()));
    /// ```
    fn set_header<K: IntoHeaderName>(&mut self, name: K, value: &str);

    /// Sends the body of the response.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to parse the body from.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use http_body_util::Full;
    /// use hyper::StatusCode;
    /// use crate::{context::NgynContext, transformer::Transformer, NgynResponse, ToBytes};
    ///
    /// struct MyResponse {
    ///     body: Full,
    /// }
    ///
    /// let mut response = MyResponse { body: Full::new(vec![1, 2, 3]) };
    /// response.send(vec![4, 5, 6]);
    /// assert_eq!(response.body.as_slice(), &[4, 5, 6]);
    /// ```
    fn send(&mut self, item: impl ToBytes);
}

impl FullResponse for NgynResponse {
    fn set_status(&mut self, status: u16) {
        *self.status_mut() = StatusCode::from_u16(status).unwrap();
    }

    fn set_header<K: IntoHeaderName>(&mut self, name: K, value: &str) {
        self.headers_mut().insert(name, value.parse().unwrap());
    }

    fn send(&mut self, item: impl ToBytes) {
        *self.body_mut() = item.to_bytes().into();
    }
}

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
