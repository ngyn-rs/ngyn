use http_body_util::Full;
use hyper::StatusCode;

use crate::{context::NgynContext, transformer::Transformer, NgynResponse, ToBytes};

/// Trait representing a full response.
pub trait FullResponse {
    /// Sets the status code of the response.
    ///
    /// # Arguments
    ///
    /// * `status` - The status code to set.
    ///
    /// # Examples
    ///
    /// ```
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

    /// Sends the body of the response.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to parse the body from.
    ///
    /// # Examples
    ///
    /// ```
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

    fn send(&mut self, item: impl ToBytes) {
        *self.body_mut() = Full::new(item.to_bytes());
    }
}

impl Transformer for NgynResponse {
    fn transform(_cx: &mut NgynContext, res: &mut NgynResponse) -> Self {
        res.clone()
    }
}
