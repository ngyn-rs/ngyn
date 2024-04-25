use http_body_util::Full;
use hyper::StatusCode;

use crate::{context::NgynContext, transformer::Transformer, NgynResponse, ParseBody};

/// Trait representing a full response.
pub trait FullResponse {
    /// Sets the status code of the response.
    ///
    /// # Arguments
    ///
    /// * `status` - The status code to set.
    ///
    /// # Returns
    ///
    /// A mutable reference to `Self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use http_body_util::Full;
    /// use hyper::StatusCode;
    /// use crate::{context::NgynContext, transformer::Transformer, NgynResponse, ParseBody};
    ///
    /// struct MyResponse {
    ///     status: StatusCode,
    /// }
    ///
    /// let mut response = MyResponse { status: StatusCode::OK };
    /// response.set_status(200);
    /// assert_eq!(response.status, StatusCode::OK);
    /// ```
    fn set_status(&mut self, status: u16) -> &mut Self;

    /// Peeks at the response body.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to parse the body from.
    ///
    /// # Returns
    ///
    /// A mutable reference to `Self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use http_body_util::Full;
    /// use hyper::StatusCode;
    /// use crate::{context::NgynContext, transformer::Transformer, NgynResponse, ParseBody};
    ///
    /// struct MyResponse {
    ///     body: Full,
    /// }
    ///
    /// let mut response = MyResponse { body: Full::new(vec![1, 2, 3]) };
    /// response.peek(vec![4, 5, 6]);
    /// assert_eq!(response.body.as_slice(), &[4, 5, 6]);
    /// ```
    fn peek(&mut self, item: impl ParseBody) -> &mut Self;
}

impl FullResponse for NgynResponse {
    fn set_status(&mut self, status: u16) -> &mut Self {
        *self.status_mut() = StatusCode::from_u16(status).unwrap();
        self
    }

    fn peek(&mut self, item: impl ParseBody) -> &mut Self {
        *self.body_mut() = Full::new(item.parse_body());
        self
    }
}

impl Transformer for NgynResponse {
    fn transform(_cx: &mut NgynContext, res: &mut NgynResponse) -> Self {
        res.clone()
    }
}
